use async_trait::async_trait;

use crate::{
    modules::auth::{ports::Repository, AuthError, OAuthAuthorization},
    utils::postgres::PostgresRepository,
};

#[async_trait]
impl Repository for PostgresRepository {
    async fn upsert_oauth(&self, authorization: &OAuthAuthorization) -> Result<(), AuthError> {
        let query = "
            INSERT INTO oauth_authorizations (user_id, provider_id, provider_user_id, access_token, refresh_token, expires_in, scope, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
            ON CONFLICT (provider_user_id) DO UPDATE
            SET access_token = EXCLUDED.access_token, refresh_token = EXCLUDED.refresh_token, expires_in = EXCLUDED.expires_in, scope = EXCLUDED.scope, updated_at = NOW()
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, OAuthAuthorization>(query)
            .bind(&authorization.user_id)
            .bind(&authorization.provider_id)
            .bind(&authorization.provider_user_id)
            .bind(&authorization.access_token)
            .bind(&authorization.refresh_token)
            .bind(&authorization.expires_in)
            .bind(&authorization.scope)
            .fetch_one(&*self.pg_pool)
            .await;

        match result {
            Ok(_record) => Ok(()),
            Err(e) => {
                log::error!("Failed to upsert oauth authorization: {}", e);
                Err(AuthError::from(e))
            }
        }
    }
}
