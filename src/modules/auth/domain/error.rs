use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Authentication failed with provider {0}")]
    AuthenticationFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JWT creation failed: {0}")]
    JwtCreationFailed(String),

    #[error(transparent)]
    JwtError(#[from] JwtError),

    #[error("Token exchange failed: {0}")]
    TokenExchangeError(String),

    #[error("Invalid token: {0}")]
    InvalidTokenError(String),

    #[error("OAuth2 request token error: {0}")]
    OAuth2RequestTokenError(String),
}
