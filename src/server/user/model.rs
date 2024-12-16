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
    pub status: i8,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
