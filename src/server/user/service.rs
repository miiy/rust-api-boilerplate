use super::dto;
use super::error::UserError;
use super::model::User;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn user(username: &str, pool: &MySqlPool) -> Result<dto::UserResponse, UserError> {
        let user_option = User::find_by_username(&pool, username).await?;

        if let Some(user) = user_option {
            let resp = dto::UserResponse {
                username: user.username,
                status: user.status,
                created_at: user
                    .created_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                updated_at: user
                    .updated_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            };
            Ok(resp)
        } else {
            Err(UserError::NotFound)
        }
    }
}
