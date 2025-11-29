use dtt_core::ports::{FeedAlgorithm, CacheService};
use dtt_core::domain::{Post, FeedScore, UserInteraction, EngagementMetrics};
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct EnhancedFeedAlgorithm {
    interactions: Arc<Mutex<Vec<UserInteraction>>>,
    cache: Arc<Mutex<HashMap<Uuid, Vec<Post>>>>,
}

impl EnhancedFeedAlgorithm {
    pub fn new() -> Self {
        Self {
            interactions: Arc::new(Mutex::new(Vec::new())),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn calculate_engagement_score(&self, post: &Post) -> f64 {
        let interactions = self.interactions.lock().unwrap();
        let post_interactions: Vec<_> = interactions.iter()
            .filter(|i| i.post_id == post.id)
            .collect();
        
        let likes = post_interactions.iter().filter(|i| matches!(i.interaction_type, dtt_core::domain::InteractionType::Like)).count() as f64;
        let views = post_interactions.iter().filter(|i| matches!(i.interaction_type, dtt_core::domain::InteractionType::View)).count() as f64;
        let shares = post_interactions.iter().filter(|i| matches!(i.interaction_type, dtt_core::domain::InteractionType::Share)).count() as f64;
        
        (likes * 3.0 + shares * 5.0) / (views + 1.0)
    }

    fn calculate_recency_score(&self, post: &Post) -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let age_hours = (now - post.timestamp) / 3600;
        1.0 / (1.0 + age_hours as f64 / 24.0)
    }
}

#[async_trait]
impl FeedAlgorithm<String> for EnhancedFeedAlgorithm {
    async fn generate_feed(&self, user_id: &Uuid, limit: usize) -> Result<Vec<Post>, String> {
        // Check cache first
        if let Some(cached) = self.get_cached_feed(user_id).await? {
            return Ok(cached.into_iter().take(limit).collect());
        }
        
        // Generate new feed (simplified - would query database in real implementation)
        let posts = vec![];
        self.cache_feed(user_id, posts.clone()).await?;
        Ok(posts)
    }

    async fn calculate_post_score(&self, post: &Post, _user_id: &Uuid) -> Result<FeedScore, String> {
        let engagement_score = self.calculate_engagement_score(post);
        let recency_score = self.calculate_recency_score(post);
        let relevance_score = 0.5; // Would be calculated based on user preferences
        
        let final_score = engagement_score * 0.4 + recency_score * 0.3 + relevance_score * 0.3;
        
        Ok(FeedScore {
            post_id: post.id,
            engagement_score,
            recency_score,
            relevance_score,
            final_score,
        })
    }

    async fn record_interaction(&self, interaction: UserInteraction) -> Result<(), String> {
        let mut interactions = self.interactions.lock().unwrap();
        interactions.push(interaction);
        Ok(())
    }
}

#[async_trait]
impl CacheService<String> for EnhancedFeedAlgorithm {
    async fn get_cached_feed(&self, user_id: &Uuid) -> Result<Option<Vec<Post>>, String> {
        let cache = self.cache.lock().unwrap();
        Ok(cache.get(user_id).cloned())
    }

    async fn cache_feed(&self, user_id: &Uuid, posts: Vec<Post>) -> Result<(), String> {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(*user_id, posts);
        Ok(())
    }

    async fn invalidate_user_cache(&self, user_id: &Uuid) -> Result<(), String> {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(user_id);
        Ok(())
    }
}