use super::model::{Post, PostStatus};
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Post {
    pub async fn create(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO posts (`user_id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&item.user_id)
        .bind(&item.category_id)
        .bind(&item.title)
        .bind(&item.author)
        .bind(&item.source)
        .bind(&item.source_url)
        .bind(&item.thumbnail)
        .bind(&item.summary)
        .bind(&item.content)
        .bind(&item.status)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Post>, sqlx::Error> {
        let item: Option<Post> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `created_at`, `updated_at`
        FROM `posts`
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn find_count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    pub async fn find_all(
        pool: &MySqlPool,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Post>, sqlx::Error> {
        let items: Vec<Post> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`
        FROM `posts`
        WHERE `status` = ? AND `deleted_at` IS NULL
        ORDER BY `created_at` DESC
        LIMIT ? OFFSET ?
        ",
        )
        .bind(PostStatus::Published.as_i8())
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn update(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE posts SET `category_id` = ?, `title` = ?, `author` = ?, `source` = ?, `source_url` = ?, `thumbnail` = ?, `summary` = ?, `content` = ?, `updated_at` = ?
        WHERE `id`=?
        ",
        )
            .bind(&item.category_id)
            .bind(&item.title)
            .bind(&item.author)
            .bind(&item.source)
            .bind(&item.source_url)
            .bind(&item.thumbnail)
            .bind(&item.summary)
            .bind(&item.content)
            .bind(&item.updated_at)
            .bind(&item.id)
            .execute(pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn soft_delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE posts SET `deleted_at` = ?
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
