use crate::{
    constants::{BPS_FACTOR, SHA_256_HASH_LENTH_BYTES},
    dto::QuotePayload,
};

klever_sc::imports!();
klever_sc::derive_imports!();

#[klever_sc::module]
pub trait ShieldXUtils {
    fn paginate<T: Clone + TopEncode + TopDecode + ManagedVecItem>(
        &self,
        items: VecMapper<Self::Api, T>,
        page: usize,
        page_size: usize,
    ) -> ManagedVec<Self::Api, T> {
        if page == 0 || page_size == 0 {
            return ManagedVec::new();
        }

        let start = (page - 1) * page_size;

        items.iter().skip(start).take(page_size).collect()
    }

    fn get_quote_hash(
        &self,
        quote: &QuotePayload<Self::Api>,
    ) -> ManagedByteArray<Self::Api, SHA_256_HASH_LENTH_BYTES> {
        let mut buf = ManagedBuffer::new();

        buf.append(quote.validator.as_managed_buffer());
        buf.append_bytes(&(quote.covered_event.clone() as u64).to_be_bytes());
        buf.append_bytes(&quote.duration_epochs.to_be_bytes());
        buf.append(&quote.payout.to_bytes_be_buffer());
        buf.append(&quote.premium.to_bytes_be_buffer());
        buf.append_bytes(&quote.expiry.to_be_bytes());
        buf.append_bytes(&quote.nonce.to_be_bytes());
        buf.append(&quote.risk_model_version);

        self.crypto().sha256(&buf)
    }

    fn bps_value(&self, value: &BigUint, bps: &u32) -> BigUint<Self::Api> {
        value * *bps / BPS_FACTOR
    }
}
