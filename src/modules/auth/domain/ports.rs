use super::{AuthError, OAuthAuthorization};
use async_trait::async_trait;
use oauth2::{basic::BasicTokenType, CsrfToken, EmptyExtraTokenFields, StandardTokenResponse};

use serde_json::Value;

#[async_trait]
pub trait Provider: Send + Sync {
    /// Generates the URL to which the user should be redirected to initiate the OAuth flow.
    async fn get_authorization_url(&self) -> (String, CsrfToken);

    /// Handles the exchange of the authorization code for an access token.
    async fn exchange_token(
        &self,
        code: String,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, AuthError>;

    /// Fetches the user data using the access token.
    async fn fetch_user_info(&self, access_token: String) -> Result<Value, reqwest::Error>;

    async fn provider_id(&self) -> i32;
}

#[async_trait]
pub trait Repository: Send + Sync {
    async fn upsert_oauth(&self, auth: &OAuthAuthorization) -> Result<(), AuthError>;
}
