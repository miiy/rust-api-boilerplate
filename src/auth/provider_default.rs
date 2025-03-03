use super::provider::{AuthenticatedUser, AuthenticationProvider, ProviderError};

#[derive(Clone)]
pub struct DefaultAuthenticationProvider {
    db: sqlx::MySqlPool,
    redis: redis::Client,
}

impl DefaultAuthenticationProvider {
    pub fn new(db: sqlx::MySqlPool, redis: redis::Client) -> Self {
        Self { db, redis }
    }
}

impl AuthenticationProvider for DefaultAuthenticationProvider {
    async fn get_user(&self, sub: String) -> Result<AuthenticatedUser, ProviderError> {
        let row: Option<(u64, String)> = sqlx::query_as(
            "
        SELECT `id`, `username`
        FROM `users`
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(sub)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = row {
            Ok(AuthenticatedUser {
                id: row.0,
                username: row.1,
            })
        } else {
            Err(ProviderError::UserNotFound)
        }
    }
}
