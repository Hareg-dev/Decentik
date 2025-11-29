use crate::domain::Post;
use uuid::Uuid;

pub trait VideoRepository<T> {
    async fn save_post(&self, post: Post) -> Result<(), T>;
    async fn get_post(&self, id: Uuid) -> Result<Option<Post>, T>;
    async fn get_user_posts(&self, user_id: Uuid) -> Result<Vec<Post>, T>;
}