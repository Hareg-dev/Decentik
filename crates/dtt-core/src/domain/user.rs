use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
}