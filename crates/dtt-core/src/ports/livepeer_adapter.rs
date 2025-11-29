use crate::domain::MediaID;

pub trait LivepeerAdapter<T> {
    async fn upload_video(&self, video_data: Vec<u8>) -> Result<MediaID, T>;
    async fn get_video_url(&self, media_id: &MediaID) -> Result<String, T>;
}