use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
}

#[async_trait]
pub trait RateLimiter<E> {
    async fn check_rate_limit(&self, user_id: &Uuid, action: &str) -> Result<bool, E>;
    async fn record_request(&self, user_id: &Uuid, action: &str) -> Result<(), E>;
}