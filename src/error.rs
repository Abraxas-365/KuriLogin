use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use jsonwebtoken::errors::Error as JwtError;
use serde_json::json;
use thiserror::Error;

use crate::modules::{auth::AuthError, user::UserError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("General authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("General user management error: {0}")]
    UserError(#[from] UserError),

    #[error("Unexpected error")]
    Unexpected,

    #[error(transparent)]
    JwtError(#[from] JwtError),

    #[error("Network error: {0}")]
    NetworkError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_message) = match self {
            AppError::AuthError(auth_error) => match auth_error {
                AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".to_string()),
                AuthError::AuthenticationFailed(provider) => (
                    StatusCode::UNAUTHORIZED,
                    format!("Authentication failed with provider {}", provider),
                ),
                AuthError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database interaction failed".to_string(),
                ),
                AuthError::NetworkError(_) => (
                    StatusCode::BAD_GATEWAY,
                    "Network error occurred".to_string(),
                ),
                AuthError::JwtCreationFailed(msg) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("JWT creation failed: {}", msg),
                ),
                AuthError::JwtError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "JWT processing error".to_string(),
                ),
                AuthError::TokenExchangeError(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("Token exchange failed: {}", msg),
                ),
                AuthError::InvalidTokenError(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid token detail: {}", msg),
                ),
                AuthError::OAuth2RequestTokenError(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("OAuth2 request token error: {}", msg),
                ),
            },
            AppError::UserError(user_error) => match user_error {
                UserError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
                UserError::InvalidUserData(msg) => (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid user data: {}", msg),
                ),
                UserError::DatabaseError(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error in user operation".to_string(),
                ),
                UserError::Unauthorized => (
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized access attempt".to_string(),
                ),
            },
            AppError::Unexpected => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An unexpected error occurred".to_string(),
            ),
            AppError::JwtError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT error encountered".to_string(),
            ),
            AppError::NetworkError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
        };

        HttpResponse::build(status_code).json(json!({ "error": error_message }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::AuthError(auth_error) => match auth_error {
                AuthError::InvalidToken => StatusCode::BAD_REQUEST,
                AuthError::AuthenticationFailed(_) => StatusCode::UNAUTHORIZED,
                AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::NetworkError(_) => StatusCode::BAD_GATEWAY,
                AuthError::JwtCreationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::TokenExchangeError(_) => StatusCode::BAD_REQUEST,
                AuthError::InvalidTokenError(_) => StatusCode::BAD_REQUEST,
                AuthError::OAuth2RequestTokenError(_) => StatusCode::BAD_REQUEST,
            },
            AppError::UserError(user_error) => match user_error {
                UserError::UserNotFound => StatusCode::NOT_FOUND,
                UserError::InvalidUserData(_) => StatusCode::BAD_REQUEST,
                UserError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                UserError::Unauthorized => StatusCode::UNAUTHORIZED,
            },
            AppError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
        }
    }
}
