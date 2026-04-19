klever_sc::imports!();

use crate::data::Policy;

#[klever_sc::module]
pub trait ShieldxEvents {
    #[event("policy_created")]
    fn emit_policy_created(
        &self,
        #[indexed] owner: ManagedAddress<Self::Api>,
        #[indexed] validator: ManagedAddress<Self::Api>,
        policy: &Policy<Self::Api>,
    );
}
