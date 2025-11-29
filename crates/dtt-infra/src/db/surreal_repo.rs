use dtt_core::{ports::VideoRepository, domain::Post};
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

pub struct SurrealVideoRepository {
    db: Surreal<Client>,
}

impl SurrealVideoRepository {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

impl VideoRepository<String> for SurrealVideoRepository {
    async fn save_post(&self, post: Post) -> Result<(), String> {
        let _: Vec<Post> = self.db
            .create("posts")
            .content(&post)
            .await
            .map_err(|e| e.to_string())?;
        
        // Create engagement edges for feed algorithm
        let _: surrealdb::Result<surrealdb::Response> = self.db
            .query("RELATE $user->posted->$post SET timestamp = time::now()")
            .bind(("user", format!("users:{}", post.user_id)))
            .bind(("post", format!("posts:{}", post.id)))
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }

    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, String> {
        let mut result: Vec<Post> = self.db
            .select(("posts", id.to_string()))
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(result.pop())
    }

    async fn get_user_posts(&self, user_id: Uuid) -> Result<Vec<Post>, String> {
        let posts: Vec<Post> = self.db
            .query("SELECT * FROM posts WHERE user_id = $user_id ORDER BY timestamp DESC LIMIT 50")
            .bind(("user_id", user_id))
            .await
            .map_err(|e| e.to_string())?
            .take(0)
            .map_err(|e| e.to_string())?;
        
        Ok(posts)
    }
}