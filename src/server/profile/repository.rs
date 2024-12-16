use super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    pub async fn find_by_username(
        pool: &MySqlPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `username`, `email`, `phone`, `status`, `created_at`, `updated_at`
        FROM `users`
        WHERE `username`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
