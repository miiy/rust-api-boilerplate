use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Default, Debug, FromRow)]
#[sqlx(default)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
    pub status: i8,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostStatus {
    Draft = 0,
    Published = 1,
    Disabled = 2,
}

#[allow(dead_code)]
impl PostStatus {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            0 => Some(Self::Draft),
            1 => Some(Self::Published),
            2 => Some(Self::Disabled),
            _ => None,
        }
    }

    pub fn as_i8(&self) -> i8 {
        *self as i8
    }
}
