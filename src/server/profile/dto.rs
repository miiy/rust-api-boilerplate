use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// profile

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub username: String,
    pub email: String,
    pub phone: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
}
