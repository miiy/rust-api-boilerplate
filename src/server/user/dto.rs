use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// user

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}
