use super::service;
use crate::error::APIError;
use crate::auth::provider::AuthenticatedUser;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde::Serialize;
use time::OffsetDateTime;

// user
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub is_self: bool,
    #[serde(with = "time::serde::iso8601::option")]
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ActivityResponse {}

#[derive(Debug, Serialize)]
pub struct FollowersResponse {}

#[derive(Debug, Serialize)]
pub struct FollowingResponse {}

// GET /users/{username}
pub async fn user(
    username: web::Path<String>,
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let resp = service::get_user_by_username(&username, &app_state.db)
        .await
        .map_err(APIError::from)?;

    let user: AuthenticatedUser = user.unwrap().into_inner();
    let is_self = resp.id == user.id;

    let resp = UserResponse {
        username: resp.username,
        is_self: is_self,
        created_at: resp.created_at,
    };
    Ok(HttpResponse::Ok().json(resp))
}

// GET /users/{username}/activity
pub async fn activity() -> Result<HttpResponse, Error> {
    let resp = ActivityResponse {};
    log::info!("activity");
    Ok(HttpResponse::Ok().json(resp))
}

// GET /users/{username}/followers
pub async fn followers() -> Result<HttpResponse, Error> {
    let resp = FollowersResponse {};
    log::info!("followers");
    Ok(HttpResponse::Ok().json(resp))
}

// GET /users/{username}/following
pub async fn following() -> Result<HttpResponse, Error> {
    let resp = FollowingResponse {};
    log::info!("following");
    Ok(HttpResponse::Ok().json(resp))
}
