use klever_sc::derive_imports::*;
use klever_sc::imports::*;

use crate::constants::ORACLE_PUBKEY_SIZE_BYTES;
use crate::data;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug)]
pub struct QuotePayload<M: ManagedTypeApi> {
    pub validator: ManagedAddress<M>,
    pub covered_event: data::CoveredEvent,
    pub duration_epochs: u64,
    pub payout: BigUint<M>,
    pub premium: BigUint<M>,
    pub expiry: u64,
    pub nonce: u64,
    pub risk_model_version: ManagedBuffer<M>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, PartialEq, Eq, Debug)]
pub struct ProtocolConfig<M: ManagedTypeApi> {
    pub oracle_pubkey: ManagedByteArray<M, ORACLE_PUBKEY_SIZE_BYTES>,
    pub protocol_fee_bps: u32,
    pub max_payout_bps_of_pool: u32,
}
