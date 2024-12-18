use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

// https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#default
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
#[sqlx(default)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub status: i8,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Disabled = 0,
    Enabled = 1,
}

#[allow(dead_code)]
impl UserStatus {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            0 => Some(Self::Disabled),
            1 => Some(Self::Enabled),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> i8 {
        *self as i8
    }
}
