use dtt_core::ports::SignatureVerifier;
use dtt_core::domain::{Post, CryptoSignature, verify_signature};
use async_trait::async_trait;

#[derive(Clone)]
pub struct CryptoSignatureService;

impl CryptoSignatureService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SignatureVerifier<String> for CryptoSignatureService {
    async fn verify_post_signature(&self, post: &Post) -> Result<bool, String> {
        if let Some(signature) = &post.signature {
            let post_data = serde_json::to_vec(post)
                .map_err(|e| format!("Serialization error: {}", e))?;
            Ok(verify_signature(&post_data, signature))
        } else {
            Ok(false)
        }
    }

    async fn verify_signature(&self, data: &[u8], signature: &CryptoSignature) -> Result<bool, String> {
        Ok(verify_signature(data, signature))
    }
}