use crate::error::{APIError, ErrorEntity};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ProfileError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("service error: {_0}")]
    Service(String),
    #[display("user not found")]
    NotFound,
}

impl ProfileError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Params(_) => 10001,
            Self::Service(_) => 10002,
            Self::NotFound => 10005,
        }
    }
}


impl From<ProfileError> for APIError {
    fn from(from: ProfileError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            ProfileError::Params(_) => APIError::BadRequest(e),
            ProfileError::Service(_) => APIError::InternalError(e),
            ProfileError::NotFound => APIError::NotFound(e),
        }
    }
}
