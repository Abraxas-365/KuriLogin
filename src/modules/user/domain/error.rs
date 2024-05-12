use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,

    #[error("User data is invalid: {0}")]
    InvalidUserData(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Unauthorized access attempt")]
    Unauthorized,
}
