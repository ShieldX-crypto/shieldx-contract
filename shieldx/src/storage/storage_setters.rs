klever_sc::imports!();

use crate::{dto::ProtocolConfig, storage};

#[klever_sc::module]
pub trait StorageSetters: storage::Storage {
    #[only_owner]
    #[endpoint(setConfig)]
    fn set_config(&self, new_config: &ProtocolConfig<Self::Api>) {
        require!(new_config.protocol_fee_bps > 0, "Protocol fee required");
        require!(
            new_config.max_payout_bps_of_pool <= 5_000,
            "Max payout too high"
        );

        self.config().set(new_config);
    }

    #[only_owner]
    #[endpoint(addOracleAddress)]
    fn add_oracle_address(&self, oracle: &ManagedAddress<Self::Api>) {
        self.oracle_whitelist().add(oracle);
    }

    #[only_owner]
    #[endpoint(removeOracleAddress)]
    fn remove_oracle_address(&self, oracle: &ManagedAddress<Self::Api>) {
        self.oracle_whitelist().remove(oracle);
    }
}
