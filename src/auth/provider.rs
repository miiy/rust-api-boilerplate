use derive_more::Display;
use serde::Serialize;
use std::future::Future;

// authenticated user
#[derive(Debug, Serialize, Clone)]
pub struct AuthenticatedUser {
    pub id: u64,
    pub username: String,
}

// provider
pub trait AuthenticationProvider {
    fn get_user(
        &self,
        sub: String,
    ) -> impl Future<Output = Result<AuthenticatedUser, ProviderError>> + Send;
    // fn get_user(&self, sub: String) -> Result<AuthenticatedUser, ProviderError>;
}

// error
#[derive(Debug, Display)]
pub enum ProviderError {
    #[display("user not found")]
    UserNotFound,
    #[display("database error: {source}")]
    DatabaseError { source: sqlx::Error },
    #[display("redis error: {source}")]
    RedisError { source: redis::RedisError },
}

impl std::error::Error for ProviderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProviderError::UserNotFound => None,
            ProviderError::DatabaseError { source } => Some(source),
            ProviderError::RedisError { source } => Some(source),
        }
    }
}

impl From<sqlx::Error> for ProviderError {
    fn from(e: sqlx::Error) -> Self {
        ProviderError::DatabaseError { source: e }
    }
}

impl From<redis::RedisError> for ProviderError {
    fn from(e: redis::RedisError) -> Self {
        ProviderError::RedisError { source: e }
    }
}
