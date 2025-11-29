use crate::domain::User;
use uuid::Uuid;

pub trait AuthService<T> {
    async fn authenticate(&self, token: &str) -> Result<User, T>;
    async fn generate_token(&self, user_id: Uuid) -> Result<String, T>;
    async fn hash_password(&self, password: &str) -> Result<String, T>;
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, T>;
}