use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Default, Debug, FromRow)]
#[sqlx(default)]
pub struct Post {
    pub id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub content: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}
