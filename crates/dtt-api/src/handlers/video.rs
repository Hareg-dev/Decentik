use axum::{extract::State, http::StatusCode, Json};
use dtt_core::{publish_video, domain::{KeyPair, Post}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct UploadRequest {
    pub user_id: Uuid,
    pub video_data: Vec<u8>,
    pub caption: Option<String>,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub post_id: Uuid,
    pub signature: String,
    pub at_uri: String,
}

#[derive(Serialize)]
pub struct FeedResponse {
    pub posts: Vec<Post>,
    pub algorithm_version: String,
}

pub async fn upload_video_handler(
    State(state): State<AppState>,
    Json(req): Json<UploadRequest>,
) -> Result<Json<UploadResponse>, StatusCode> {
    // Generate keypair for user (in production, retrieve from secure storage)
    let user_keypair = KeyPair::generate();
    
    match publish_video(
        req.video_data,
        req.user_id,
        req.caption,
        &user_keypair,
        &state.repo,
        &state.livepeer,
        &state.at_client,
    ).await {
        Ok(post) => {
            let signature_b64 = base64::encode(
                post.signature.as_ref()
                    .map(|s| &s.signature)
                    .unwrap_or(&vec![])
            );
            
            Ok(Json(UploadResponse { 
                post_id: post.id,
                signature: signature_b64,
                at_uri: format!("at://did:plc:example/app.bsky.feed.post/{}", post.id),
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_feed_handler(
    State(state): State<AppState>,
    // Extract user_id from JWT token in real implementation
) -> Result<Json<FeedResponse>, StatusCode> {
    let user_id = Uuid::new_v4(); // Placeholder
    
    match state.feed_algorithm.generate_feed(user_id).await {
        Ok(posts) => Ok(Json(FeedResponse {
            posts,
            algorithm_version: "v1.0-graph-engagement".to_string(),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}