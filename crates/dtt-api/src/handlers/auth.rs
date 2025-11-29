use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public endpoints
    let path = request.uri().path();
    if path.starts_with("/auth/") || path == "/health" {
        return Ok(next.run(request).await);
    }

    // Extract JWT token from Authorization header
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match state.auth.authenticate(token).await {
                    Ok(_user) => {
                        // User authenticated, proceed
                        return Ok(next.run(request).await);
                    }
                    Err(_) => {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}