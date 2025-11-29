use crate::domain::Post;
use crate::ports::VideoRepository;
use uuid::Uuid;

pub async fn get_for_you_feed<R, E>(
    user_id: Uuid,
    repo: &R,
) -> Result<Vec<Post>, E>
where
    R: VideoRepository<E>,
{
    repo.get_user_posts(user_id).await
}