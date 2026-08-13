klever_sc::imports!();

use crate::data::{CoveredEvent, Policy};
use crate::dto::ProtocolConfig;

#[klever_sc::module]
pub trait Storage {
    #[storage_mapper("oracleWhitelist")]
    fn oracle_whitelist(&self) -> WhitelistMapper<ManagedAddress>;

    #[storage_mapper("policies")]
    fn policies(&self, policy_id: u64) -> SingleValueMapper<Policy<Self::Api>>;

    #[storage_mapper("policiesBySubject")]
    fn policies_by_subject(
        &self,
        event_type: &CoveredEvent,
        subject_key: &ManagedBuffer<Self::Api>,
    ) -> VecMapper<u64>;

    #[storage_mapper("policiesByExpiryEpoch")]
    fn policies_by_expiry_epoch(&self, expiry_epoch: u64) -> VecMapper<u64>;

    #[storage_mapper("ownedPolicies")]
    fn owned_policies(&self, owner: &ManagedAddress<Self::Api>) -> VecMapper<u64>;

    #[storage_mapper("usedNonces")]
    fn used_nonces(&self) -> SetMapper<u64>;

    #[storage_mapper("nextPolicyId")]
    fn next_policy_id(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("subjectEvents")]
    fn subject_events(
        &self,
        event_type: &CoveredEvent,
        subject_key: &ManagedBuffer<Self::Api>,
    ) -> UnorderedSetMapper<u64>;

    #[storage_mapper("config")]
    fn config(&self) -> SingleValueMapper<ProtocolConfig<Self::Api>>;

    #[storage_mapper("poolBalance")]
    fn pool_balance(&self) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("lockedLiquidity")]
    fn locked_liquidity(&self) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("lockedLiquidityBySubject")]
    fn locked_liquidity_by_subject(
        &self,
        event_type: &CoveredEvent,
        subject_key: &ManagedBuffer<Self::Api>,
    ) -> SingleValueMapper<BigUint<Self::Api>>;

    #[storage_mapper("protocolBalance")]
    fn protocol_balance(&self) -> SingleValueMapper<BigUint<Self::Api>>;
}
