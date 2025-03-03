use super::model::{Article, ArticleStatus};
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Article {
    const TABLE_NAME: &str = "articles";

    pub async fn create(pool: &MySqlPool, item: &Article) -> Result<u64, sqlx::Error> {
        let query = format!(
            "
        INSERT INTO {} (`user_id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ",
            Self::TABLE_NAME
        );
        sqlx::query(&query)
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

    pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Article>, sqlx::Error> {
        let query = format!(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `created_at`, `updated_at`
        FROM {}
        WHERE `id`=? AND `deleted_at` IS NULL
        ",
            Self::TABLE_NAME
        );

        let item: Option<Article> = sqlx::query_as(&query)
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn find_count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let query = format!(
            "
        SELECT COUNT(*) FROM {} WHERE deleted_at IS NULL
        ",
            Self::TABLE_NAME
        );
        let count: i64 = sqlx::query_scalar(&query)
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    pub async fn find_all(
        pool: &MySqlPool,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Article>, sqlx::Error> {
        let query = format!(
            "
        SELECT `id`, `category_id`, `title`, `author`, `source`, `source_url`, `thumbnail`, `summary`, `content`, `status`, `created_at`, `updated_at`
        FROM {}
        WHERE `status` = ? AND `deleted_at` IS NULL
        ORDER BY `created_at` DESC
        LIMIT ? OFFSET ?
        ",
            Self::TABLE_NAME
        );
        let items: Vec<Article> = sqlx::query_as(&query)
            .bind(ArticleStatus::Published.as_i8())
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok(items)
    }

    pub async fn update(pool: &MySqlPool, item: &Article) -> Result<u64, sqlx::Error> {
        let query = format!(
            "
        UPDATE {} SET `category_id` = ?, `title` = ?, `author` = ?, `source` = ?, `source_url` = ?, `thumbnail` = ?, `summary` = ?, `content` = ?, `updated_at` = ?
        WHERE `id`=?
        ",
            Self::TABLE_NAME
        );
        sqlx::query(&query)
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

    pub async fn soft_delete(pool: &MySqlPool, id: u64, user_id: u64) -> Result<u64, sqlx::Error> {
        let query = format!(
            "
        UPDATE {} SET `deleted_at` = ?
        WHERE `id`=? AND `user_id`=? AND `deleted_at` IS NULL
        ",
            Self::TABLE_NAME
        );
        sqlx::query(&query)
            .bind(OffsetDateTime::now_utc())
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await
            .map(|x| x.rows_affected())
    }
}
