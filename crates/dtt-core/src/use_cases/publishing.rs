use crate::domain::{Post, KeyPair};
use crate::ports::{VideoRepository, LivepeerAdapter, ATClient};
use uuid::Uuid;

pub async fn publish_video<R, L, A, E>(
    video_data: Vec<u8>,
    user_id: Uuid,
    caption: Option<String>,
    user_keypair: &KeyPair,
    repo: &R,
    livepeer: &L,
    at_client: &A,
) -> Result<Post, E>
where
    R: VideoRepository<E>,
    L: LivepeerAdapter<E>,
    A: ATClient<E>,
{
    let media_id = livepeer.upload_video(video_data).await?;
    
    let mut post = Post {
        id: Uuid::new_v4(),
        user_id,
        media_id,
        caption,
        signature: None,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    // Create cryptographic signature
    let post_data = serde_json::to_vec(&post).unwrap();
    let signature = user_keypair.sign(&post_data);
    post.signature = Some(signature);
    
    repo.save_post(post.clone()).await?;
    at_client.publish_to_at(&post).await?;
    
    Ok(post)
}