use klever_sc::derive_imports::*;
use klever_sc::imports::*;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem)]
pub enum CoveredEvent {
    Jailed,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem)]
pub enum PolicyStatus {
    Active,
    Claimed,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug, ManagedVecItem)]
pub struct Policy<M: ManagedTypeApi> {
    pub id: u64,
    pub owner_address: ManagedAddress<M>,
    pub validator: ManagedAddress<M>,
    pub covered_event: CoveredEvent,
    pub premium: BigUint<M>,
    pub payout: BigUint<M>,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub nonce: u64,
    pub status: PolicyStatus,

    pub risk_model_version: ManagedBuffer<M>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug)]
pub struct ValidatorEvent {
    event: CoveredEvent,
    epoch: u64,
}
