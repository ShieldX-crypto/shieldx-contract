klever_sc::imports!();

use crate::data::{Policy, ValidatorEvent};

#[klever_sc::module]
pub trait ShieldxEvents {
    #[event("policyCreated")]
    fn emit_policy_created(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] validator: ManagedAddress<Self::Api>,
        policy: &Policy<Self::Api>,
    );

    #[event("validatorEventRegistered")]
    fn emit_validator_event_registered(
        &self,
        #[indexed] oracle: ManagedAddress<Self::Api>,
        #[indexed] validator: ManagedAddress<Self::Api>,
        event: &ValidatorEvent<Self::Api>,
    );

    #[event("payoutGranted")]
    fn emit_payout_granted(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] validator: ManagedAddress<Self::Api>,
        #[indexed] epoch: u64,
        amount: BigUint<Self::Api>,
    );

    #[event("policyExpired")]
    fn emit_policy_expired(
        &self,
        #[indexed] validator: ManagedAddress<Self::Api>,
        #[indexed] epoch: u64,
        policy: &Policy<Self::Api>,
    );
}
