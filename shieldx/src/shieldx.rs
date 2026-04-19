#![no_std]

use klever_sc::types::{BigInt, BigUint};

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

/// An empty contract. To be used as a template when starting a new contract from scratch.
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
    fn init(&self) {
        self.pool_balance().set_if_empty(BigUint::zero());
        self.protocol_balance().set_if_empty(BigUint::zero());
        self.locked_liquidity().set_if_empty(BigUint::zero());
    }

    #[upgrade]
    fn upgrade(&self) {}
}
