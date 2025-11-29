use axum::{routing::{get, post}, Router, middleware};
use crate::{handlers, AppState};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(handlers::video::upload_video_handler))
        .route("/feed", get(handlers::video::get_feed_handler))
        .route("/profile/:id", get(handlers::user::get_profile_handler))
        .route("/auth/login", post(handlers::user::login_handler))
        .route("/auth/register", post(handlers::user::register_handler))
        .layer(middleware::from_fn_with_state(
            (), 
            handlers::auth::auth_middleware
        ))
}