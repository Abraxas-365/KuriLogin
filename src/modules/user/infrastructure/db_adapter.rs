use async_trait::async_trait;

use crate::{
    modules::user::{ports::Repository, User, UserError},
    utils::postgres::PostgresRepository,
};

#[async_trait]
impl Repository for PostgresRepository {
    async fn upsert_user(&self, user: &User) -> Result<User, UserError> {
        let query = "
            INSERT INTO users (email, name, avatar_url, created_at, updated_at)
            VALUES ($1, $2, $3, NOW(), NOW())
            ON CONFLICT (email) DO UPDATE
            SET name = EXCLUDED.name, avatar_url = EXCLUDED.avatar_url, updated_at = NOW()
            RETURNING *;
        ";
        sqlx::query_as::<_, User>(query)
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.avatar_url)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<User, UserError> {
        let query = "
            SELECT * FROM users WHERE user_id = $1;
        ";
        sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_one(&*self.pg_pool)
            .await
            .map_err(UserError::from)
    }
}
