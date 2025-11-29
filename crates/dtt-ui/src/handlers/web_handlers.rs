use axum::{
    extract::{Path, State},
    response::Html,
    http::StatusCode,
};
use askama_axum::Template;
use uuid::Uuid;
use crate::templates::{FeedTemplate, UploadTemplate, ProfileTemplate};
use crate::adapters::UIService;
use dtt_core::ports::{VideoRepository, FeedAlgorithm, SignatureVerifier, RateLimiter};

pub async fn feed_handler<R, F, S, L>(
    State(ui_service): State<UIService<R, F, S, L>>,
) -> Result<Html<String>, StatusCode>
where
    R: VideoRepository<String> + Clone,
    F: FeedAlgorithm<String> + Clone,
    S: SignatureVerifier<String> + Clone,
    L: RateLimiter<String> + Clone,
{
    let user_id = Uuid::new_v4(); // In real app, get from session
    
    match ui_service.get_user_feed(&user_id).await {
        Ok(posts) => {
            let template = FeedTemplate {
                posts,
                user_id: user_id.to_string(),
            };
            Ok(Html(template.render().unwrap()))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn upload_handler() -> Html<String> {
    let template = UploadTemplate {
        user_id: Uuid::new_v4().to_string(),
    };
    Html(template.render().unwrap())
}

pub async fn profile_handler<R, F, S, L>(
    Path(user_id): Path<String>,
    State(ui_service): State<UIService<R, F, S, L>>,
) -> Result<Html<String>, StatusCode>
where
    R: VideoRepository<String> + Clone,
    F: FeedAlgorithm<String> + Clone,
    S: SignatureVerifier<String> + Clone,
    L: RateLimiter<String> + Clone,
{
    let user_uuid = Uuid::parse_str(&user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match ui_service.get_user_feed(&user_uuid).await {
        Ok(posts) => {
            let template = ProfileTemplate {
                user_id,
                posts,
            };
            Ok(Html(template.render().unwrap()))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}