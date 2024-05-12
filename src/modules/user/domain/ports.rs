use async_trait::async_trait;

use super::{User, UserError};

#[async_trait]
pub trait Repository: Send + Sync {
    async fn upsert_user(&self, user: &User) -> Result<User, UserError>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<User, UserError>;
}
