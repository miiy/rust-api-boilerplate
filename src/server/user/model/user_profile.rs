use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
#[sqlx(default)]
pub struct UserProfile {
    pub id: u64,
    pub user_id: u64,
    pub nickname: String,
    pub avatar: String,
    pub current_status: String,
    pub bio: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}