use async_trait::async_trait;
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
};

use serde_json::Value;

use crate::modules::auth::{ports::Provider, AuthError};

pub struct GoogleProvider {
    client: BasicClient,
}

impl GoogleProvider {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URI"));

        GoogleProvider { client }
    }
}

#[async_trait]
impl Provider for GoogleProvider {
    async fn get_authorization_url(&self) -> (String, CsrfToken) {
        let scopes = vec!["email", "profile", "openid"];
        let (auth_url, csrf_token) = scopes
            .iter()
            .fold(
                self.client.authorize_url(CsrfToken::new_random),
                |url, scope| url.add_scope(Scope::new(scope.to_string())),
            )
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent")
            .url();

        (auth_url.to_string(), csrf_token)
    }

    async fn exchange_token(
        &self,
        code: String,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, AuthError> {
        self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|err| {
                log::error!("Failed to exchange code: {:?}", err);
                AuthError::OAuth2RequestTokenError(err.to_string())
            })
    }

    async fn fetch_user_info(&self, access_token: String) -> Result<Value, reqwest::Error> {
        let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
        reqwest::Client::new()
            .get(user_info_url)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    async fn provider_id(&self) -> i32 {
        1 // Represents Google as an OAuth provider in your system
    }
}
