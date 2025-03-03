use super::error::ProfileError;
use super::service;
use crate::auth::provider::AuthenticatedUser;
use crate::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
// profile

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub current_status: String,
    pub bio: String,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: String,
    pub avatar: String,
    pub current_status: String,
    pub bio: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

#[derive(Debug, Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

// GET /profile
pub async fn profile(
    user: Option<web::ReqData<AuthenticatedUser>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if let Some(user_data) = user {
        let user = user_data.into_inner();
        let req = service::ProfileRequest {
            username: user.username,
        };
        let resp = service::profile(req, &app_state.db)
            .await
            .map_err(APIError::from)?;
        let resp = ProfileResponse {
            username: resp.username,
            nickname: resp.nickname,
            avatar: resp.avatar,
            current_status: resp.current_status,
            bio: resp.bio,
            created_at: resp.created_at,
        };
        Ok(HttpResponse::Ok().json(resp))
    } else {
        return Err(APIError::from(ProfileError::NotFound).into());
    }
}

pub async fn update_profile(
    req: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, Error> {
    log::info!("update_profile: {}, {}, {}, {}", req.nickname, req.avatar, req.current_status, req.bio);
    let resp = UpdateProfileResponse {
        message: "Profile updated successfully".to_string(),
    };
    Ok(HttpResponse::Ok().json(resp))
}

// PUT /profile/password
pub async fn change_password(
    req: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, Error> {
    log::info!("change_password: {}, {}, {}", req.current_password, req.new_password, req.new_password_confirmation);
    let resp = ChangePasswordResponse {
        message: "Password changed successfully".to_string(),
    };
    Ok(HttpResponse::Ok().json(resp))
}
