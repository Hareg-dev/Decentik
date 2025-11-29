use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::crypto::CryptoSignature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: MediaID,
    pub caption: Option<String>,
    pub signature: Option<CryptoSignature>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaID {
    pub ipfs_hash: String,
    pub livepeer_job_id: Option<String>,
}