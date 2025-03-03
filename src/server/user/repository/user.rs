use super::super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    const TABLE_NAME: &str = "users";

    pub async fn find(
        pool: &MySqlPool,
        id: u64,
    ) -> Result<Option<User>, sqlx::Error> {
        let query = format!(
            "
        SELECT `id`, `username`, `status`, `created_at`, `updated_at`
        FROM `{}`
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
            Self::TABLE_NAME
        );
        let item: Option<User> = sqlx::query_as(&query)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(item)
    }

    pub async fn find_by_username(
        pool: &MySqlPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let query = format!(
            "
        SELECT `id`, `username`, `email`, `phone`, `status`, `created_at`, `updated_at`
        FROM `{}`
        WHERE `username`=? AND `deleted_at` IS NULL
        ",
            Self::TABLE_NAME
        );
        let item: Option<User> = sqlx::query_as(&query)
            .bind(username)
            .fetch_optional(pool)
            .await?;
        Ok(item)
    }

}
