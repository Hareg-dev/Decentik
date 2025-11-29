use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub post_id: Uuid,
    pub views: u64,
    pub likes: u64,
    pub shares: u64,
    pub comments: u64,
    pub watch_time_seconds: u64,
    pub completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub interaction_type: InteractionType,
    pub timestamp: u64,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    View,
    Like,
    Share,
    Comment,
    Follow,
    Skip,
}

#[derive(Debug, Clone)]
pub struct FeedScore {
    pub post_id: Uuid,
    pub relevance_score: f64,
    pub engagement_score: f64,
    pub recency_score: f64,
    pub final_score: f64,
}