use crate::domain::{Post, CryptoSignature};
use async_trait::async_trait;

#[async_trait]
pub trait SignatureVerifier<E> {
    async fn verify_post_signature(&self, post: &Post) -> Result<bool, E>;
    async fn verify_signature(&self, data: &[u8], signature: &CryptoSignature) -> Result<bool, E>;
}