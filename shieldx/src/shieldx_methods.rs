klever_sc::imports!();

use crate::{
    constants::ERR_DOES_NOT_EXIST,
    constants::ORACLE_SIGNATURE_SIZE_BYTES,
    data::{CoveredEvent, Policy, PolicyStatus, ValidatorEvent},
    dto::QuotePayload,
    events, storage, utils,
};

#[klever_sc::module]
pub trait ShieldxMethods: storage::Storage + utils::ShieldXUtils + events::ShieldxEvents {
    #[only_owner]
    #[endpoint(withdrawLiquidity)]
    fn withdraw_liquidity(&self, to: ManagedAddress, amount: BigUint) {
        let pool = self.pool_balance().get();
        let locked = self.locked_liquidity().get();
        let protocol = self.protocol_balance().get();

        let available = if pool > locked.clone() + protocol.clone() {
            pool.clone() - locked - protocol
        } else {
            BigUint::zero()
        };

        require!(available >= amount, "Insufficient available liquidity");
        self.pool_balance().set(pool - &amount);
        self.send().direct_klv(&to, &amount);
    }

    #[payable("KLV")]
    #[endpoint(depositLiquidity)]
    fn deposit_liquidity(&self) {
        let payment = self.call_value().klv_value().clone_value();
        require!(payment > 0, "Payment is required");

        let pool = self.pool_balance().get();
        self.pool_balance().set(pool + payment);
    }

    #[payable("KLV")]
    #[endpoint(buyPolicy)]
    fn buy_policy(
        &self,
        payload: &QuotePayload<Self::Api>,
        oracle_signature: &ManagedByteArray<Self::Api, ORACLE_SIGNATURE_SIZE_BYTES>,
    ) {
        let payment = self.call_value().klv_value().clone_value();
        require!(payment.eq(&payload.premium), "Premium mismatch");
        require!(
            payload.covered_event == CoveredEvent::Jailed,
            "Unsupported event"
        );
        require!(
            payload.expiry >= self.blockchain().get_block_timestamp(),
            "Quote expired"
        );
        require!(
            !self.used_nonces().contains(&payload.nonce),
            "Nonce already used"
        );

        let quote_hash = self.get_quote_hash(payload);

        let config = self.config().get();

        let oracle_pubkey = config.oracle_pubkey;

        self.crypto().verify_ed25519(
            oracle_pubkey.as_managed_buffer(),
            quote_hash.as_managed_buffer(),
            oracle_signature.as_managed_buffer(),
        );

        let pool = self.pool_balance().get();
        let max_allowed = self.bps_value(&pool, &config.max_payout_bps_of_pool);
        require!(payload.payout <= max_allowed, "Payout exceeds pool limit");

        let protocol_fee = self.bps_value(&payload.premium, &config.protocol_fee_bps);
        let pool_part = payload.premium.clone() - protocol_fee.clone();
        self.pool_balance().update(|v| *v += pool_part);
        self.protocol_balance().update(|v| *v += protocol_fee);
        self.locked_liquidity().update(|v| *v += &payload.payout);

        let policy_id = self.next_policy_id().get();
        self.next_policy_id().set(policy_id + 1);

        let start_epoch = self.blockchain().get_block_epoch();
        let end_epoch = start_epoch + payload.duration_epochs;

        let policy = Policy {
            id: policy_id,
            owner_address: self.blockchain().get_caller(),
            validator: payload.validator.clone(),
            covered_event: payload.covered_event.clone(),
            premium: payload.premium.clone(),
            payout: payload.payout.clone(),
            start_epoch: start_epoch,
            end_epoch: end_epoch,
            nonce: payload.nonce,
            status: PolicyStatus::Active,
            risk_model_version: payload.risk_model_version.clone(),
        };

        self.policies(policy_id).set(policy.clone());
        self.policies_by_validator(&policy.validator)
            .push(&policy_id);
        self.policies_by_expiry_epoch(end_epoch).push(&policy_id);
        self.used_nonces().insert(policy.nonce);

        self.emit_policy_created(
            policy.owner_address.clone(),
            policy.validator.clone(),
            &policy,
        );
    }

    #[endpoint(registerValidatorEvent)]
    fn register_validator_event(&self, event: &ValidatorEvent<Self::Api>) {
        let caller = self.blockchain().get_caller();
        require!(
            self.oracle_whitelist().contains(&caller),
            "Unauthorized oracle address"
        );

        require!(
            event.event == CoveredEvent::Jailed,
            "Unsupported event type"
        );
        require!(
            event.epoch <= self.blockchain().get_block_epoch(),
            "Event epoch is invalid. Should not be in the future"
        );

        self.validator_events(&event.validator).insert(event.epoch);
        self.emit_validator_event_registered(caller, event.validator.clone(), event);
    }

    #[endpoint(batchClaim)]
    fn batch_claim(&self, policy_ids: &ManagedVec<Self::Api, u64>, trigger_epoch: u64) {
        require!(
            trigger_epoch <= self.blockchain().get_block_epoch(),
            "Invalid trigger epoch. Should not be in the future"
        );

        for id in policy_ids {
            let policy_opt = self.policies(id);
            require!(!policy_opt.is_empty(), ERR_DOES_NOT_EXIST);
            let mut policy = policy_opt.get();

            require!(
                policy.status == PolicyStatus::Active,
                "Policy is not claimable"
            );
            require!(
                self.blockchain().get_block_epoch() <= policy.end_epoch,
                "Policy is expired"
            );
            require!(
                trigger_epoch >= policy.start_epoch && trigger_epoch <= policy.end_epoch,
                "Trigger outside covergae window"
            );
            require!(
                self.validator_events(&policy.validator)
                    .contains(&trigger_epoch),
                "Event for trigger event is not registered"
            );

            policy.status = PolicyStatus::Claimed;
            self.policies(id).set(policy.clone());

            self.send()
                .direct_klv(&policy.owner_address, &policy.payout);

            self.emit_payout_granted(
                policy.owner_address,
                policy.validator,
                trigger_epoch,
                policy.payout,
            );
        }
    }

    #[endpoint(expirePolicies)]
    fn expire_policies(&self, epoch: u64) {
        let policy_ids = self.policies_by_expiry_epoch(epoch);

        for policy_id in policy_ids.iter() {
            let policy_opt = self.policies(policy_id);
            if policy_opt.is_empty() {
                continue;
            }

            let mut policy = policy_opt.get();
            if policy.status != PolicyStatus::Active {
                continue;
            }
            require!(
                policy.end_epoch < self.blockchain().get_block_epoch(),
                "Policy is not expired yet"
            );
            policy.status = PolicyStatus::Expired;
            self.policies(policy_id).set(policy.clone());
            let payout = policy.payout.clone();
            self.locked_liquidity().update(|v| *v -= payout);
            self.emit_policy_expired(policy.validator.clone(), epoch, &policy);
        }
    }
}
