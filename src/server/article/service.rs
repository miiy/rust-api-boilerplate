use super::error::ArticleError;
use super::model::{Article, ArticleStatus};
use crate::pagination::Pagination;
use sqlx::MySqlPool;
use time::OffsetDateTime;

// list
#[derive(Debug)]
pub struct ListRequest {
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug)]
pub struct ListResponse {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub data: Vec<ListResponseItem>,
}

#[derive(Debug)]
pub struct ListResponseItem {
    pub id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

// detail
#[derive(Debug)]
pub struct DetailRequest {
    pub id: u64,
}

#[derive(Debug)]
pub struct DetailResponse {
    pub id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

// create
#[derive(Debug)]
pub struct CreateRequest {
    pub user_id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
}

#[derive(Debug)]
pub struct CreateResponse {
    pub id: u64,
}

// update
#[derive(Debug)]
pub struct UpdateRequest {
    pub id: u64,
    pub user_id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
}

#[derive(Debug)]
pub struct UpdateResponse {
    pub id: u64,
}

// delete
#[derive(Debug)]
pub struct DeleteRequest {
    pub id: u64,
    pub user_id: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeleteResponse {
    pub row_affected: u64,
}

pub async fn lists(
    req: ListRequest,
    pool: &MySqlPool,
) -> Result<ListResponse, ArticleError> {
    let total = Article::find_count(&pool).await?;
    let pg = Pagination::new(req.page, req.per_page, total);

    let offset = pg.offset();
    let articles = Article::find_all(&pool, pg.per_page, offset).await?;
    let results: Vec<ListResponseItem> = articles
        .into_iter()
        .map(|article| ListResponseItem {
            id: article.id,
            category_id: article.category_id,
            title: article.title,
            author: article.author,
            source: article.source,
            source_url: article.source_url,
            thumbnail: article.thumbnail,
            summary: article.summary,
            created_at: article.created_at,
            updated_at: article.updated_at,
        })
        .collect();

    Ok(ListResponse {
        page: pg.page,
        per_page: pg.per_page,
        total_pages: pg.total_pages,
        data: results,
    })
}

pub async fn detail(req: DetailRequest, pool: &MySqlPool) -> Result<DetailResponse, ArticleError> {
    let article = Article::find(&pool, req.id)
        .await?
        .ok_or(ArticleError::NotFound)?;

    let resp = DetailResponse {
        id: article.id,
        category_id: article.category_id,
        title: article.title,
        author: article.author,
        source: article.source,
        source_url: article.source_url,
        thumbnail: article.thumbnail,
        summary: article.summary,
        content: article.content,
        created_at: article.created_at,
        updated_at: article.updated_at,
    };
    Ok(resp)
}

pub async fn create(
    req: CreateRequest,
    pool: &MySqlPool,
) -> Result<CreateResponse, ArticleError> {
    let now = OffsetDateTime::now_utc();
    let article = Article {
        id: 0,
        user_id: req.user_id,
        category_id: req.category_id,
        title: req.title,
        author: req.author,
        source: req.source,
        source_url: req.source_url,
        thumbnail: req.thumbnail,
        summary: req.summary,
        content: req.content,
        status: ArticleStatus::Published.as_i8(),
        created_at: Some(now),
        updated_at: Some(now),
    };
    let article_id = Article::create(&pool, &article).await?;

    let resp = CreateResponse { id: article_id };
    Ok(resp)
}

pub async fn update(
    req: UpdateRequest,
    pool: &MySqlPool,
) -> Result<UpdateResponse, ArticleError> {
    let article = Article {
        id: req.id,
        user_id: req.user_id,
        category_id: req.category_id,
        title: req.title,
        author: req.author,
        source: req.source,
        source_url: req.source_url,
        thumbnail: req.thumbnail,
        summary: req.summary,
        content: req.content,
        status: ArticleStatus::Published.as_i8(),
        created_at: None,
        updated_at: Some(OffsetDateTime::now_utc()),
    };
    let article_id = Article::update(&pool, &article).await?;

    Ok(UpdateResponse { id: article_id })
}

pub async fn delete(req: DeleteRequest, pool: &MySqlPool) -> Result<DeleteResponse, ArticleError> {
    let r = Article::soft_delete(&pool, req.id, req.user_id).await?;
    log::info!("article {} deleted by user {}", req.id, req.user_id);
    Ok(DeleteResponse {
        row_affected: r,
    })
}
