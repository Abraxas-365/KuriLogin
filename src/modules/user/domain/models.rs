use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: i32,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserBuilder {
    user_id: Option<i32>,
    email: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user_id: None,
            email: None,
            name: None,
            avatar_url: None,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn user_id(mut self, user_id: i32) -> Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn email<S: Into<String>>(mut self, email: S) -> Self {
        self.email = Some(email.into());
        self
    }
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn avatar_url<S: Into<String>>(mut self, avatar_url: S) -> Self {
        self.avatar_url = Some(avatar_url.into());
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
    pub fn build(self) -> User {
        User {
            user_id: self.user_id.unwrap_or(0),
            email: self.email,
            name: self.name,
            avatar_url: self.avatar_url,
            created_at: self.created_at.unwrap_or_else(Utc::now),
            updated_at: self.updated_at.unwrap_or_else(Utc::now),
        }
    }
}
