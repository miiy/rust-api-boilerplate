use super::dto::*;
use super::error::PostError;
use super::model::{Post, PostStatus};
use crate::pagination::Pagination;
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn lists(
        page: u32,
        per_page: u32,
        pool: &MySqlPool,
    ) -> Result<ListResponse, PostError> {
        let total = Post::find_count(&pool).await?;
        let pg = Pagination::new(page, per_page, total);

        let offset = pg.offset();
        let posts = Post::find_all(&pool, pg.per_page, offset).await?;
        let results: Vec<ListResponseItem> = posts
            .into_iter()
            .map(|post| ListResponseItem {
                id: post.id,
                category_id: post.category_id,
                title: post.title,
                author: post.author,
                source: post.source,
                source_url: post.source_url,
                thumbnail: post.thumbnail,
                summary: post.summary,
                created_at: post
                    .created_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
                updated_at: post
                    .updated_at
                    .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            })
            .collect();

        Ok(ListResponse {
            page: pg.page,
            per_page: pg.per_page,
            total_pages: pg.total_pages,
            data: results,
        })
    }

    pub async fn detail(id: u64, pool: &MySqlPool) -> Result<DetailResponse, PostError> {
        let post = Post::find(&pool, id).await?.ok_or(PostError::NotFound)?;

        let resp = DetailResponse {
            id: post.id,
            category_id: post.category_id,
            title: post.title,
            author: post.author,
            source: post.source,
            source_url: post.source_url,
            thumbnail: post.thumbnail,
            summary: post.summary,
            content: post.content,
            created_at: post
                .created_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
            updated_at: post
                .updated_at
                .unwrap_or_else(|| OffsetDateTime::from_unix_timestamp(0).unwrap()),
        };
        Ok(resp)
    }

    pub async fn create(req: CreateRequest, pool: &MySqlPool) -> Result<CreateResponse, PostError> {
        let now = OffsetDateTime::now_utc();
        let post = Post {
            id: 0,
            user_id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            source: req.source,
            source_url: req.source_url,
            thumbnail: req.thumbnail,
            summary: req.summary,
            content: req.content,
            status: PostStatus::Published.as_i8(),
            created_at: Some(now),
            updated_at: Some(now),
        };
        let post_id = Post::create(&pool, &post).await?;

        let resp = CreateResponse { id: post_id };
        Ok(resp)
    }

    pub async fn update(
        id: u64,
        req: UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, PostError> {
        let post = Post {
            id: id,
            user_id: 0,
            category_id: req.category_id,
            title: req.title,
            author: req.author,
            source: req.source,
            source_url: req.source_url,
            thumbnail: req.thumbnail,
            summary: req.summary,
            content: req.content,
            status: PostStatus::Published.as_i8(),
            created_at: None,
            updated_at: Some(OffsetDateTime::now_utc()),
        };
        let post_id = Post::update(&pool, &post).await?;

        Ok(UpdateResponse { id: post_id })
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<DeleteResponse, PostError> {
        let _ra = Post::soft_delete(&pool, id).await?;
        Ok(DeleteResponse {})
    }
}
