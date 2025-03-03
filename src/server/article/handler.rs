use super::error::ArticleError;
use super::service;

use crate::error::APIError;
use crate::AppState;
use crate::auth::provider::AuthenticatedUser;
use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// list
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub data: Vec<ListResponseItem>,
}

#[derive(Debug, Serialize)]
pub struct ListResponseItem {
    pub id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}

// detail
#[derive(Debug, Serialize)]
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
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::iso8601::option")]
    pub updated_at: Option<OffsetDateTime>,
}

// create
#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub id: u64,
}

// update
#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub source: String,
    pub source_url: String,
    pub thumbnail: String,
    pub summary: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub id: u64,
}

// delete
// pub struct DeleteResponse;
// json: null
//
// pub struct DeleteResponse{}
// json: {}
#[derive(Debug, Serialize)]
pub struct DeleteResponse {}


// GET /articles
pub async fn list(
    req: HttpRequest,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = web::Query::<HashMap<String, String>>::from_query(req.query_string())?;
    let page = query
        .get("page")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(20);
    let req = service::ListRequest {
        page: page,
        per_page: page_size,
    };
    let resp = service::lists(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = ListResponse {
        page: resp.page,
        per_page: resp.per_page,
        total_pages: resp.total_pages,
        data: resp.data.into_iter().map(|item| ListResponseItem {
            id: item.id,
            category_id: item.category_id,
            title: item.title,
            author: item.author,
            source: item.source,
            source_url: item.source_url,
            thumbnail: item.thumbnail,
            summary: item.summary,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }).collect(),
    };

    Ok(HttpResponse::Ok().json(resp))
}

// GET /articles/{id}
pub async fn detail(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(ArticleError::Params(e.to_string())))?;
    let req = service::DetailRequest {
        id: id,
    };
    let resp = service::detail(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = DetailResponse {
        id: resp.id,
        category_id: resp.category_id,
        title: resp.title,
        author: resp.author,
        source: resp.source,
        source_url: resp.source_url,
        thumbnail: resp.thumbnail,
        summary: resp.summary,
        content: resp.content,
        created_at: resp.created_at,
        updated_at: resp.updated_at,
    };
    Ok(HttpResponse::Ok().json(resp))
}

// POST /articles
pub async fn create(
    req: web::Json<CreateRequest>,
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let req = req.into_inner();
    let user = user.unwrap().into_inner();
    let req = service::CreateRequest {
        user_id: user.id,
        category_id: req.category_id,
        title: req.title,
        author: req.author,
        source: req.source,
        source_url: req.source_url,
        thumbnail: req.thumbnail,
        summary: req.summary,
        content: req.content,
    };
    let resp = service::create(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = CreateResponse {
        id: resp.id,
    };
    Ok(HttpResponse::Created().json(resp))
}

// PUT /articles/{id}
pub async fn update(
    path: web::Path<String>,
    req: web::Json<UpdateRequest>,
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(ArticleError::Params(e.to_string())))?;
    let req = req.into_inner();
    let user = user.unwrap().into_inner();
    let req = service::UpdateRequest {
        id: id,
        user_id: user.id,
        category_id: req.category_id,
        title: req.title,
        author: req.author,
        source: req.source,
        source_url: req.source_url,
        thumbnail: req.thumbnail,
        summary: req.summary,
        content: req.content,
    };
    let resp = service::update(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = UpdateResponse {
        id: resp.id,
    };
    Ok(HttpResponse::Ok().json(resp))
}

// DELETE /articles/{id}
pub async fn delete(
    path: web::Path<String>,
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path
        .into_inner()
        .parse::<u64>()
        .map_err(|e| APIError::from(ArticleError::Params(e.to_string())))?;
    let user = user.unwrap().into_inner();
    let req = service::DeleteRequest {
        id: id,
        user_id: user.id,
    };
    let _resp = service::delete(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = DeleteResponse {};
    Ok(HttpResponse::Ok().json(resp))
}
