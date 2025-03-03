use super::super::model::UserProfile;
use sqlx::mysql::MySqlPool;

impl UserProfile {
    const TABLE_NAME: &str = "user_profiles";

    pub async fn get_user_profile(
        pool: &MySqlPool,
        user_id: u64,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let query = format!(
            "
        SELECT `id`, `nickname`, `avatar`, `current_status`, `bio`, `created_at`, `updated_at`
        FROM `{}`
        WHERE `user_id`=? AND `deleted_at` IS NULL
        ",
            Self::TABLE_NAME
        );
        let item: Option<UserProfile> = sqlx::query_as(&query)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(item)
    }
}
