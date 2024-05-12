use std::sync::Arc;

use crate::{error::AppError, modules::auth::JwtManager};

use super::{ports::Repository, User};

pub struct AppService {
    repo: Arc<dyn Repository>,
    jwt_manager: Arc<JwtManager>,
}

impl AppService {
    pub fn new(repo: Arc<dyn Repository>, jwt_manager: Arc<JwtManager>) -> Self {
        Self { repo, jwt_manager }
    }
}

impl AppService {
    pub async fn upsert_user(&self, user: &User) -> Result<User, AppError> {
        Ok(self.repo.upsert_user(user).await?)
    }

    pub async fn get_user_by_id(&self, token: &str) -> Result<User, AppError> {
        let claim = self.jwt_manager.verify_jwt(token)?;
        Ok(self.repo.get_user_by_id(claim.sub).await?)
    }
}
