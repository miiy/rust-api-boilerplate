use super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    pub async fn find(pool: &MySqlPool, user_id: u64) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `username`, `email`, `phone`, `status`, `created_at`, `updated_at`
        FROM `users`
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
