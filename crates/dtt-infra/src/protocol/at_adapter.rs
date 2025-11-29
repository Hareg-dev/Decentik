use dtt_core::{ports::ATClient, domain::Post};
use reqwest::Client;
use serde_json::json;
use base64::{Engine as _, engine::general_purpose};

pub struct ATProtocolAdapter {
    client: Client,
    pds_url: String,
    did: String,
    signing_key: Vec<u8>,
}

impl ATProtocolAdapter {
    pub fn new(pds_url: String, did: String, signing_key: Vec<u8>) -> Self {
        Self {
            client: Client::new(),
            pds_url,
            did,
            signing_key,
        }
    }
    
    fn create_jwt(&self, payload: serde_json::Value) -> Result<String, String> {
        // Simplified JWT creation for AT Protocol
        let header = json!({
            "alg": "EdDSA",
            "typ": "JWT"
        });
        
        let header_b64 = general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_string(&header).unwrap());
        let payload_b64 = general_purpose::URL_SAFE_NO_PAD
            .encode(serde_json::to_string(&payload).unwrap());
        
        let message = format!("{}.{}", header_b64, payload_b64);
        
        // Sign with Ed25519 (simplified)
        let signature = general_purpose::URL_SAFE_NO_PAD
            .encode(&self.signing_key[..32]); // Placeholder signature
        
        Ok(format!("{}.{}", message, signature))
    }
}

impl ATClient<String> for ATProtocolAdapter {
    async fn publish_to_at(&self, post: &Post) -> Result<String, String> {
        let record = json!({
            "$type": "app.bsky.feed.post",
            "text": post.caption.as_deref().unwrap_or(""),
            "embed": {
                "$type": "app.bsky.embed.video",
                "video": {
                    "ref": {
                        "$link": post.media_id.ipfs_hash
                    },
                    "mimeType": "video/mp4"
                }
            },
            "createdAt": chrono::Utc::now().to_rfc3339(),
            "signature": post.signature
        });
        
        let jwt_payload = json!({
            "iss": self.did,
            "aud": self.pds_url,
            "exp": chrono::Utc::now().timestamp() + 3600
        });
        
        let jwt = self.create_jwt(jwt_payload)?;
        
        let response = self.client
            .post(&format!("{}/xrpc/com.atproto.repo.createRecord", self.pds_url))
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&json!({
                "repo": self.did,
                "collection": "app.bsky.feed.post",
                "record": record
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(format!("at://{}/app.bsky.feed.post/{}", 
            self.did, 
            result["uri"].as_str().unwrap_or("unknown")
        ))
    }

    async fn sync_from_at(&self, user_handle: &str) -> Result<Vec<Post>, String> {
        let response = self.client
            .get(&format!("{}/xrpc/app.bsky.feed.getAuthorFeed", self.pds_url))
            .query(&[("actor", user_handle)])
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        // Parse AT Protocol posts back to our Post format
        let posts = result["feed"].as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|item| {
                // Convert AT Protocol post to our Post struct
                // This is a simplified conversion
                None // Placeholder
            })
            .collect();
        
        Ok(posts)
    }
}