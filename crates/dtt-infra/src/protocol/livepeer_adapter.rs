use dtt_core::{ports::LivepeerAdapter, domain::MediaID};
use reqwest::Client;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose};

pub struct LivepeerService {
    client: Client,
    api_key: String,
    ipfs_gateway: String,
}

impl LivepeerService {
    pub fn new(api_key: String, ipfs_gateway: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            ipfs_gateway,
        }
    }
}

impl LivepeerAdapter<String> for LivepeerService {
    async fn upload_video(&self, video_data: Vec<u8>) -> Result<MediaID, String> {
        // Upload to IPFS first
        let ipfs_response = self.client
            .post(&format!("{}/api/v0/add", self.ipfs_gateway))
            .multipart(reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::bytes(video_data)
                    .file_name("video.mp4")
                    .mime_str("video/mp4").unwrap()))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let ipfs_result: serde_json::Value = ipfs_response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        let ipfs_hash = ipfs_result["Hash"]
            .as_str()
            .ok_or("No IPFS hash returned")?;
        
        // Create Livepeer transcoding job
        let livepeer_response = self.client
            .post("https://livepeer.studio/api/asset/request-upload")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "name": format!("video-{}", uuid::Uuid::new_v4()),
                "source": {
                    "type": "url",
                    "url": format!("{}/ipfs/{}", self.ipfs_gateway, ipfs_hash)
                }
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let livepeer_result: serde_json::Value = livepeer_response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        let job_id = livepeer_result["id"]
            .as_str()
            .map(|s| s.to_string());
        
        Ok(MediaID {
            ipfs_hash: ipfs_hash.to_string(),
            livepeer_job_id: job_id,
        })
    }

    async fn get_video_url(&self, media_id: &MediaID) -> Result<String, String> {
        if let Some(job_id) = &media_id.livepeer_job_id {
            // Get Livepeer playback URL
            let response = self.client
                .get(&format!("https://livepeer.studio/api/asset/{}", job_id))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await
                .map_err(|e| e.to_string())?;
            
            let result: serde_json::Value = response
                .json()
                .await
                .map_err(|e| e.to_string())?;
            
            if let Some(playback_url) = result["playbackUrl"].as_str() {
                Ok(playback_url.to_string())
            } else {
                // Fallback to IPFS
                Ok(format!("{}/ipfs/{}", self.ipfs_gateway, media_id.ipfs_hash))
            }
        } else {
            // Direct IPFS access
            Ok(format!("{}/ipfs/{}", self.ipfs_gateway, media_id.ipfs_hash))
        }
    }
}