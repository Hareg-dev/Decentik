use axum::{routing::get, Router};
use dtt_infra::{
    db::{SurrealClient, SurrealVideoRepository},
    protocol::{ATProtocolAdapter, LivepeerService},
    auth::JwtAuthService,
    services::EnhancedFeedAlgorithm,
    security::{CryptoSignatureService, InMemoryRateLimiter},
};
use dtt_ui::{adapters::UIService, handlers::*};
use dtt_core::domain::{SecureKeyManager, KeyRotationPolicy};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, cors::CorsLayer};
use std::env;

#[derive(Clone)]
pub struct EnhancedAppState {
    pub ui_service: UIService<SurrealVideoRepository, EnhancedFeedAlgorithm, CryptoSignatureService, InMemoryRateLimiter>,
    pub key_manager: SecureKeyManager,
}

#[tokio::main]
async fn main() {
    // Load configuration
    let db_url = env::var("SURREAL_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    // Initialize secure key management
    let key_rotation_policy = KeyRotationPolicy {
        rotation_interval_days: 30,
        max_key_age_days: 90,
    };
    let key_manager = SecureKeyManager::new(key_rotation_policy);
    
    // Initialize database
    let db = SurrealClient::connect(&db_url).await
        .expect("Failed to connect to SurrealDB");
    
    // Initialize services with security enhancements
    let repo = SurrealVideoRepository::new(db.clone());
    let feed_algo = EnhancedFeedAlgorithm::new();
    let signature_verifier = CryptoSignatureService::new();
    let rate_limiter = InMemoryRateLimiter::new();
    
    let ui_service = UIService::new(repo, feed_algo, signature_verifier, rate_limiter);
    
    let state = EnhancedAppState {
        ui_service,
        key_manager,
    };

    let app = Router::new()
        // Web UI routes
        .route("/", get(|| async { axum::response::Redirect::permanent("/feed") }))
        .route("/feed", get(feed_handler))
        .route("/upload", get(upload_handler))
        .route("/profile/:id", get(profile_handler))
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        // CORS for API
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("🚀 Enhanced Decentik server running on http://0.0.0.0:3000");
    println!("🔐 Security: Enhanced cryptography + rate limiting");
    println!("🧠 Algorithm: ML-powered engagement scoring");
    println!("⚡ Performance: Multi-layer caching enabled");
    println!("🎨 UI: Mobile-first responsive design");
    
    axum::serve(listener, app).await.unwrap();
}