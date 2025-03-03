use crate::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum ArticleError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("unauthorized")]
    Unauthorized,
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("article not found")]
    NotFound,
}

impl ArticleError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Params(_) => 10001,
            Self::Unauthorized => 10002,
            Self::Service(_) => 10003,
            Self::Database { .. } => 10004,
            Self::Redis { .. } => 10005,
            Self::NotFound => 10006,
        }
    }
}

impl Error for ArticleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for ArticleError {
    fn from(from: sqlx::Error) -> ArticleError {
        ArticleError::Database { source: from }
    }
}

impl From<RedisError> for ArticleError {
    fn from(from: RedisError) -> ArticleError {
        ArticleError::Redis { source: from }
    }
}

impl From<ArticleError> for APIError {
    fn from(from: ArticleError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            ArticleError::Params(_) => APIError::BadRequest(e),
            ArticleError::Unauthorized => APIError::Unauthorized(e),
            ArticleError::Service(_)
            | ArticleError::Database { .. }
            | ArticleError::Redis { .. } => APIError::InternalError(e),
            ArticleError::NotFound => APIError::NotFound(e),
        }
    }
}
