use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::AuthError;

#[derive(FromRow)]
pub struct OAuthProvider {
    pub provider_id: i32,
    pub name: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct OAuthAuthorization {
    pub auth_id: i32,
    pub user_id: i32,
    pub provider_id: i32,
    pub provider_user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<DateTime<Utc>>,
    pub scope: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct OAuthAuthorizationBuilder {
    auth_id: Option<i32>,
    user_id: Option<i32>,
    provider_id: Option<i32>,
    provider_user_id: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<DateTime<Utc>>,
    scope: Option<Vec<String>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
impl OAuthAuthorizationBuilder {
    pub fn new() -> Self {
        Self {
            auth_id: None,
            user_id: None,
            provider_id: None,
            provider_user_id: None,
            access_token: None,
            refresh_token: None,
            expires_in: None,
            scope: None,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn auth_id(mut self, auth_id: i32) -> Self {
        self.auth_id = Some(auth_id);
        self
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn provider_id(mut self, provider_id: i32) -> Self {
        self.provider_id = Some(provider_id);
        self
    }
    pub fn provider_user_id<S: Into<String>>(mut self, provider_user_id: S) -> Self {
        self.provider_user_id = Some(provider_user_id.into());
        self
    }
    pub fn access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.access_token = Some(access_token.into());
        self
    }
    pub fn refresh_token<S: Into<String>>(mut self, refresh_token: S) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }
    pub fn expires_in(mut self, expires_in: DateTime<Utc>) -> Self {
        self.expires_in = Some(expires_in);
        self
    }
    pub fn scope(mut self, scope: Vec<String>) -> Self {
        self.scope = Some(scope);
        self
    }
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
    pub fn build(self) -> OAuthAuthorization {
        OAuthAuthorization {
            auth_id: self.auth_id.unwrap_or(0),
            user_id: self.user_id.unwrap_or(0),
            provider_id: self.provider_id.unwrap_or(0),
            provider_user_id: self.provider_user_id.unwrap_or_default(),
            access_token: self.access_token.unwrap_or_default(),
            refresh_token: self.refresh_token,
            expires_in: self.expires_in,
            scope: self.scope,
            created_at: self.created_at.unwrap_or_else(Utc::now),
            updated_at: self.updated_at.unwrap_or_else(Utc::now),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwtManager {
    secret: String,
    algorithm: Algorithm,
    validation: Validation,
}

impl JwtManager {
    // Constructor to initialize JwtManager
    pub fn new(secret: String) -> Self {
        JwtManager {
            secret,
            algorithm: Algorithm::HS256,
            validation: {
                let mut validation = Validation::new(Algorithm::HS256);
                validation.validate_exp = false;
                validation
            },
        }
    }

    // Create a JWT for a given user
    pub fn create_jwt(&self, user_id: i32) -> Result<String, AuthError> {
        let expiration_seconds = 3600; // Expiration in 1 hour
        let claims = Claims {
            sub: user_id,
            exp: (Utc::now() + chrono::Duration::seconds(expiration_seconds)).timestamp(), // Unix timestamp
        };

        encode(
            &Header::new(self.algorithm),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|err| AuthError::JwtCreationFailed(err.to_string()))
    }

    // Verify a JWT and return the associated claims
    pub fn verify_jwt(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &self.validation,
        )
        .map(|data| data.claims)
        .map_err(|err| AuthError::JwtError(err))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}
