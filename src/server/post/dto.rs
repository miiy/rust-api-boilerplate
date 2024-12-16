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
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
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
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
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
