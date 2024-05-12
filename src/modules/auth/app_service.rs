use std::sync::Arc;

use oauth2::TokenResponse;

use crate::{
    error::AppError,
    modules::user::{self, UserBuilder},
};

use super::{
    ports::{Provider, Repository},
    AuthError, JwtManager, OAuthAuthorizationBuilder,
};

pub struct AppService {
    provider: Arc<dyn Provider>,
    repo: Arc<dyn Repository>,
    user_service: Arc<user::AppService>,
    jwt_manager: Arc<JwtManager>,
}
impl AppService {
    pub fn new(
        provider: Arc<dyn Provider>,
        repo: Arc<dyn Repository>,
        user_service: Arc<user::AppService>,
        jwt_manager: Arc<JwtManager>,
    ) -> Self {
        Self {
            provider,
            repo,
            user_service,
            jwt_manager,
        }
    }
}

impl AppService {
    /// Initiates the OAuth process by generating the URL to redirect the user for authentication.
    pub async fn initiate_oauth(&self) -> Result<String, AppError> {
        let (auth_url, _) = self.provider.get_authorization_url().await; // CSRF token could be stored for future validation
        Ok(auth_url)
    }

    pub async fn oauth_login(&self, auth_code: String) -> Result<String, AppError> {
        log::debug!("Received auth code: {}", auth_code);
        let token_response = self
            .provider
            .exchange_token(auth_code)
            .await
            .map_err(|e| AppError::AuthError(AuthError::TokenExchangeError(e.to_string())))?;

        let access_token = token_response.access_token().secret().clone();

        log::debug!("Access token: {}", access_token);

        let refresh_token = token_response
            .refresh_token()
            .map(|token| token.secret().to_string())
            .ok_or_else(|| AuthError::InvalidTokenError("No refresh token received".to_string()))?;
        let user_info = self
            .provider
            .fetch_user_info(access_token.clone())
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;

        let provider_user_id =
            user_info
                .get("sub")
                .and_then(|val| val.as_str())
                .ok_or(AppError::AuthError(AuthError::AuthenticationFailed(
                    "User ID not found".into(),
                )))?;

        let user = {
            let mut user_builder = UserBuilder::new();
            if let Some(name) = user_info.get("given_name").and_then(|val| val.as_str()) {
                user_builder = user_builder.name(name);
            }

            if let Some(email) = user_info.get("email").and_then(|val| val.as_str()) {
                user_builder = user_builder.email(email);
            }

            if let Some(picture) = user_info.get("picture").and_then(|val| val.as_str()) {
                user_builder = user_builder.avatar_url(picture);
            }
            user_builder.build()
        };

        log::debug!("User info: {:?}", user);

        let user = self.user_service.upsert_user(&user).await?;

        let auth_data = {
            let mut auth_builder = OAuthAuthorizationBuilder::new()
                .user_id(user.user_id)
                .provider_id(self.provider.provider_id().await)
                .provider_user_id(provider_user_id)
                .access_token(access_token)
                .refresh_token(refresh_token)
                .created_at(chrono::Utc::now())
                .updated_at(chrono::Utc::now());

            if let Some(expires_in) = token_response.expires_in() {
                auth_builder = auth_builder.expires_in(chrono::Utc::now() + expires_in);
            }

            if let Some(scopes) = token_response.scopes() {
                auth_builder = auth_builder.scope(
                    scopes
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                );
            }

            auth_builder.build()
        };

        let _ = self.repo.upsert_oauth(&auth_data).await?;
        log::debug!("OAuth data upserted: {:?}", auth_data);

        let jwt = self.jwt_manager.create_jwt(auth_data.user_id)?;

        Ok(jwt)
    }
}
