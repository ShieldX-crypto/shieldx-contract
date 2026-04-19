klever_sc::imports!();

use crate::{
    constants::ORACLE_SIGNATURE_SIZE_BYTES,
    data::{CoveredEvent, Policy, PolicyStatus},
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

        self.emit_policy_created(policy.owner_address.clone(), policy.validator.clone(), &policy);
    }
}
