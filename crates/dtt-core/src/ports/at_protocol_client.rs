use crate::domain::Post;

pub trait ATClient<T> {
    async fn publish_to_at(&self, post: &Post) -> Result<String, T>;
    async fn sync_from_at(&self, user_handle: &str) -> Result<Vec<Post>, T>;
}