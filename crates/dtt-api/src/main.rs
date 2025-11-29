use axum::Router;
use dtt_infra::{
    db::{SurrealClient, SurrealVideoRepository},
    protocol::{ATProtocolAdapter, LivepeerService},
    auth::JwtAuthService,
    services::FeedAlgorithm,
};
use tokio::net::TcpListener;
use std::env;

mod router;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub repo: SurrealVideoRepository,
    pub at_client: ATProtocolAdapter,
    pub livepeer: LivepeerService,
    pub auth: JwtAuthService,
    pub feed_algorithm: FeedAlgorithm,
}

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let db_url = env::var("SURREAL_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let livepeer_api_key = env::var("LIVEPEER_API_KEY").unwrap_or_else(|_| "your-api-key".to_string());
    let ipfs_gateway = env::var("IPFS_GATEWAY").unwrap_or_else(|_| "http://localhost:5001".to_string());
    let at_pds_url = env::var("AT_PDS_URL").unwrap_or_else(|_| "https://bsky.social".to_string());
    let at_did = env::var("AT_DID").unwrap_or_else(|_| "did:plc:example".to_string());
    
    // Initialize database connection
    let db = SurrealClient::connect(&db_url).await
        .expect("Failed to connect to SurrealDB");
    
    let state = AppState {
        repo: SurrealVideoRepository::new(db.clone()),
        at_client: ATProtocolAdapter::new(
            at_pds_url,
            at_did,
            vec![0; 32], // Placeholder signing key
        ),
        livepeer: LivepeerService::new(livepeer_api_key, ipfs_gateway),
        auth: JwtAuthService::new(jwt_secret),
        feed_algorithm: FeedAlgorithm::new(db),
    };

    let app = router::create_router().with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("🚀 Decentralized TikTok server running on http://0.0.0.0:3000");
    println!("📊 SurrealDB: {}", db_url);
    println!("🔐 Authentication: JWT enabled");
    println!("📹 Video processing: Livepeer + IPFS");
    println!("🌐 AT Protocol integration: enabled");
    
    axum::serve(listener, app).await.unwrap();
}