use dtt_core::ports::{VideoRepository, FeedAlgorithm, SignatureVerifier, RateLimiter};
use dtt_core::domain::{Post, User, UserInteraction};
use uuid::Uuid;

#[derive(Clone)]
pub struct UIService<R, F, S, L> {
    pub repo: R,
    pub feed_algo: F,
    pub signature_verifier: S,
    pub rate_limiter: L,
}

impl<R, F, S, L> UIService<R, F, S, L>
where
    R: VideoRepository<String> + Clone,
    F: FeedAlgorithm<String> + Clone,
    S: SignatureVerifier<String> + Clone,
    L: RateLimiter<String> + Clone,
{
    pub fn new(repo: R, feed_algo: F, signature_verifier: S, rate_limiter: L) -> Self {
        Self {
            repo,
            feed_algo,
            signature_verifier,
            rate_limiter,
        }
    }

    pub async fn get_user_feed(&self, user_id: &Uuid) -> Result<Vec<Post>, String> {
        // Check rate limit
        if !self.rate_limiter.check_rate_limit(user_id, "feed").await? {
            return Err("Rate limit exceeded".to_string());
        }
        
        self.rate_limiter.record_request(user_id, "feed").await?;
        self.feed_algo.generate_feed(user_id, 20).await
    }

    pub async fn verify_and_get_post(&self, post_id: &Uuid) -> Result<Option<Post>, String> {
        if let Some(post) = self.repo.get_post(post_id).await? {
            let is_valid = self.signature_verifier.verify_post_signature(&post).await?;
            if is_valid {
                Ok(Some(post))
            } else {
                Err("Invalid post signature".to_string())
            }
        } else {
            Ok(None)
        }
    }

    pub async fn record_user_interaction(&self, interaction: UserInteraction) -> Result<(), String> {
        self.feed_algo.record_interaction(interaction).await
    }
}