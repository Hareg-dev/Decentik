use dtt_core::domain::Post;
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FeedScore {
    post_id: String,
    score: f64,
    engagement_rate: f64,
    recency_score: f64,
    creator_score: f64,
}

pub struct FeedAlgorithm {
    db: Surreal<Client>,
}

impl FeedAlgorithm {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }

    pub async fn generate_feed(&self, user_id: Uuid) -> Result<Vec<Post>, String> {
        // Complex graph query for personalized feed
        let feed_scores: Vec<FeedScore> = self.db
            .query("
                LET $user = $user_id;
                LET $now = time::now();
                
                // Get user's social graph (followers, following)
                LET $following = (SELECT ->follows->out AS user FROM $user).user;
                LET $followers = (SELECT <-follows<-in AS user FROM $user).user;
                LET $social_network = array::union($following, $followers);
                
                // Calculate engagement scores for posts
                SELECT 
                    id AS post_id,
                    (
                        // Engagement rate (likes, comments, shares)
                        (count(<-likes) * 1.0 + count(<-comments) * 2.0 + count(<-shares) * 3.0) / 
                        math::max(1, time::unix($now) - time::unix(timestamp))
                    ) AS engagement_rate,
                    
                    // Recency score (exponential decay)
                    math::exp(-0.1 * (time::unix($now) - time::unix(timestamp)) / 3600) AS recency_score,
                    
                    // Creator score (based on follower count and verification)
                    (
                        count((SELECT <-follows FROM users WHERE id = $parent.user_id)) * 0.1 +
                        (SELECT verified FROM users WHERE id = $parent.user_id LIMIT 1)[0] * 10
                    ) AS creator_score,
                    
                    // Final weighted score
                    (
                        engagement_rate * 0.4 +
                        recency_score * 0.3 +
                        creator_score * 0.2 +
                        // Boost posts from social network
                        (user_id IN $social_network ? 0.1 : 0)
                    ) AS score
                    
                FROM posts 
                WHERE 
                    // Filter out posts older than 7 days
                    timestamp > (time::unix($now) - 604800) AND
                    // Exclude user's own posts
                    user_id != $user AND
                    // Content moderation filters
                    flagged != true
                ORDER BY score DESC
                LIMIT 50
            ")
            .bind(("user_id", format!("users:{}", user_id)))
            .await
            .map_err(|e| e.to_string())?
            .take(0)
            .map_err(|e| e.to_string())?;
        
        // Fetch full post data for top scored posts
        let mut posts = Vec::new();
        for score in feed_scores {
            if let Ok(Some(post)) = self.get_post_by_id(&score.post_id).await {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn get_post_by_id(&self, post_id: &str) -> Result<Option<Post>, String> {
        let mut result: Vec<Post> = self.db
            .select(post_id)
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(result.pop())
    }
    
    pub async fn record_engagement(&self, user_id: Uuid, post_id: Uuid, engagement_type: &str) -> Result<(), String> {
        // Record engagement for algorithm learning
        let _: surrealdb::Result<surrealdb::Response> = self.db
            .query("RELATE $user->$engagement_type->$post SET timestamp = time::now(), weight = $weight")
            .bind(("user", format!("users:{}", user_id)))
            .bind(("post", format!("posts:{}", post_id)))
            .bind(("engagement_type", engagement_type))
            .bind(("weight", match engagement_type {
                "likes" => 1.0,
                "comments" => 2.0,
                "shares" => 3.0,
                "views" => 0.1,
                _ => 1.0,
            }))
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
}