klever_sc::imports!();

use crate::{dto::QuotePayload, utils};

#[klever_sc::module]
pub trait ShieldxDebug: utils::ShieldXUtils {
    #[view(quoteHash)]
    fn quote_hash_debug(&self, quote: QuotePayload<Self::Api>) -> ManagedByteArray<Self::Api, 32> {
        self.get_quote_hash(&quote)
    }
}
