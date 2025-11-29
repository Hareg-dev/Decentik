use crate::domain::{Post, FeedScore, UserInteraction};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FeedAlgorithm<E> {
    async fn generate_feed(&self, user_id: &Uuid, limit: usize) -> Result<Vec<Post>, E>;
    async fn calculate_post_score(&self, post: &Post, user_id: &Uuid) -> Result<FeedScore, E>;
    async fn record_interaction(&self, interaction: UserInteraction) -> Result<(), E>;
}

#[async_trait]
pub trait CacheService<E> {
    async fn get_cached_feed(&self, user_id: &Uuid) -> Result<Option<Vec<Post>>, E>;
    async fn cache_feed(&self, user_id: &Uuid, posts: Vec<Post>) -> Result<(), E>;
    async fn invalidate_user_cache(&self, user_id: &Uuid) -> Result<(), E>;
}