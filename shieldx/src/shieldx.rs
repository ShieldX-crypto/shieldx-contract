#![no_std]

use crate::dto::ProtocolConfig;

klever_sc::imports!();

mod constants;
#[path = "data/data.rs"]
mod data;
mod debug;
#[path = "data/dto.rs"]
mod dto;
mod events;
mod shieldx_methods;
mod shieldx_views;
#[path = "storage/storage.rs"]
mod storage;
#[path = "storage/storage_setters.rs"]
mod storage_setters;
mod utils;

#[klever_sc::contract]
pub trait Shieldx:
    storage::Storage
    + storage_setters::StorageSetters
    + shieldx_methods::ShieldxMethods
    + shieldx_views::ShieldxViews
    + utils::ShieldXUtils
    + events::ShieldxEvents
    + debug::ShieldxDebug
{
    #[init]
    fn init(
        &self,
        oracle_pubkey: ManagedByteArray<Self::Api, 32>,
        protocol_fee_bps: u32,
        max_payout_bps_of_pool: u32,
        max_locked_per_subject_bps: u32
    ) {
        self.pool_balance().set_if_empty(BigUint::zero());
        self.protocol_balance().set_if_empty(BigUint::zero());
        self.locked_liquidity().set_if_empty(BigUint::zero());

        let config = ProtocolConfig {
            oracle_pubkey,
            protocol_fee_bps,
            max_payout_bps_of_pool,
            max_locked_per_subject_bps
        };

        self.config().set_if_empty(config);
        self.next_policy_id().set_if_empty(1u64);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
