klever_sc::imports!();

use crate::{constants::ERR_DOES_NOT_EXIST, data::Policy, storage, utils};

#[klever_sc::module]
pub trait ShieldxViews: storage::Storage + utils::ShieldXUtils {
    #[view(getPolicyById)]
    fn get_policy_by_id(&self, policy_id: u64) -> Policy<Self::Api> {
        let policy_opt = self.policies(policy_id);
        require!(!policy_opt.is_empty(), ERR_DOES_NOT_EXIST);
        return policy_opt.get();
    }

    #[view(getPoliciesBatch)]
    fn get_policies_batch(&self, policy_ids: &ManagedVec<Self::Api, u64>) -> ManagedVec<Self::Api, Policy<Self::Api>> {
        let mut result = ManagedVec::<Self::Api, Policy<Self::Api>>::new();

        for id in policy_ids {
            let policy_opt = self.policies(id);
            require!(!policy_opt.is_empty(), ERR_DOES_NOT_EXIST);

            result.push(policy_opt.get());
        }

        return result;
    }

    #[view(getOwnedPoliciesPaged)]
    fn get_owned_policies_paged(&self, page: usize, page_size: usize) -> ManagedVec<Self::Api, Policy<Self::Api>> {
        let owner = self.blockchain().get_caller();
        let owned_policies = self.owned_policies(&owner);
        let ids = self.paginate(owned_policies, page, page_size);

        return self.get_policies_batch(&ids);
    }
}
