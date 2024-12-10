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

    pub async fn check_name_available(pool: &MySqlPool, name: &str) -> Result<bool, sqlx::Error> {
        let count: i32 = sqlx::query_scalar("select count(*) from users where `name` = ?")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(count == 0)
    }

    pub async fn create(pool: &MySqlPool, item: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO users (`name`, `email`, `password`, `created_at`, `updated_at`)
        VALUES(?, ?, ?, ?, ?)
        ",
        )
        .bind(&item.name)
        .bind(&item.email)
        .bind(&item.password)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn find_by_name(pool: &MySqlPool, name: String) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `password`
        FROM `users`
        WHERE `name`=? AND `delete_time` IS NULL
        ",
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
