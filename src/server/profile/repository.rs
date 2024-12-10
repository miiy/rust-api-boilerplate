use super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    pub async fn find_by_name(pool: &MySqlPool, name: &str) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `email`, `created_at`, `updated_at`
        FROM `users`
        WHERE `name`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
