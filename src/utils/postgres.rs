use std::sync::Arc;

use sqlx::PgPool;

use super::config::Config;

#[derive(Debug, Clone)]
pub struct PostgresRepository {
    pub pg_pool: Arc<PgPool>,
}

impl PostgresRepository {
    pub async fn new() -> Self {
        let config = Config::from_env();

        // Append SSL parameters to the connection URL
        let conn_url = format!("{}", config.database_url);

        let pool = PgPool::connect(&conn_url).await.unwrap();
        sqlx::migrate!("./migrations/")
            .run(&pool)
            .await
            .expect("Failed to run migrations.");

        println!("All migrations have been run successfully.");
        Self {
            pg_pool: Arc::new(pool),
        }
    }
}
