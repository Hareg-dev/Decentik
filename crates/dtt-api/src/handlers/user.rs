use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Serialize)]
pub struct ProfileResponse {
    pub user_id: Uuid,
    pub handle: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub handle: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub handle: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
}

pub async fn get_profile_handler(
    Path(id): Path<Uuid>,
) -> Result<Json<ProfileResponse>, StatusCode> {
    Ok(Json(ProfileResponse {
        user_id: id,
        handle: "example_user".to_string(),
        display_name: "Example User".to_string(),
    }))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // In production: verify credentials against database
    let user_id = Uuid::new_v4();
    
    match state.auth.generate_token(user_id).await {
        Ok(token) => Ok(Json(AuthResponse { token, user_id })),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    // Hash password and create user
    let _password_hash = state.auth.hash_password(&req.password).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let user_id = Uuid::new_v4();
    
    // In production: save user to database
    
    match state.auth.generate_token(user_id).await {
        Ok(token) => Ok(Json(AuthResponse { token, user_id })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}