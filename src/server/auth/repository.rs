use super::model::User;
use sqlx::MySqlPool;

impl User {
    pub async fn check_email_available(pool: &MySqlPool, email: &str) -> Result<bool, sqlx::Error> {
        let count: i32 = sqlx::query_scalar("select count(*) from users where `email` = ?")
            .bind(email)
            .fetch_one(pool)
            .await?;
        Ok(count == 0)
    }

    pub async fn check_username_available(
        pool: &MySqlPool,
        username: &str,
    ) -> Result<bool, sqlx::Error> {
        let count: i32 = sqlx::query_scalar("select count(*) from users where `username` = ?")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(count == 0)
    }

    pub async fn create(pool: &MySqlPool, item: &User) -> Result<u64, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // users
        let user_id = sqlx::query(
            "
        INSERT INTO users (`username`, `email`, `password`, `status`, `created_at`, `updated_at`)
        VALUES(?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.password)
        .bind(&item.status)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(&mut *tx)
        .await
        .map(|x| x.last_insert_id())?;

        // user_profile
        sqlx::query(
            "
        INSERT INTO user_profiles (`user_id`, `created_at`, `updated_at`)
        VALUES(?, ?, ?)
        ",
        )
        .bind(user_id)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(&mut *tx)
        .await?;

        // commit
        tx.commit().await?;

        Ok(user_id)
    }

    pub async fn find_by_username(
        pool: &MySqlPool,
        username: String,
    ) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `username`, `password`
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
