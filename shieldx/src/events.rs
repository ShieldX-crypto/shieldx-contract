klever_sc::imports!();

use crate::data::{CoveredEvent, Policy, OracleEvent};

#[klever_sc::module]
pub trait ShieldxEvents {
    #[event("policyCreated")]
    fn emit_policy_created(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] event: CoveredEvent,
        #[indexed] subject_key: ManagedBuffer<Self::Api>,
        policy: &Policy<Self::Api>,
    );

    #[event("oracleEventRegistered")]
    fn emit_oracle_event_registered(
        &self,
        #[indexed] oracle: ManagedAddress<Self::Api>,
        #[indexed] event: CoveredEvent,
        #[indexed] subject_key: ManagedBuffer<Self::Api>,
        registered: &OracleEvent<Self::Api>,
    );

    #[event("payoutGranted")]
    fn emit_payout_granted(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] policy_id: u64,
        #[indexed] event: CoveredEvent,
        #[indexed] subject_key: ManagedBuffer<Self::Api>,
        #[indexed] epoch: u64,
        amount: BigUint<Self::Api>,
    );

    #[event("policyExpired")]
    fn emit_policy_expired(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] policy_id: u64,
        #[indexed] event: CoveredEvent,
        #[indexed] subject_key: ManagedBuffer<Self::Api>,
        #[indexed] epoch: u64,
        policy: &Policy<Self::Api>,
    );
}
