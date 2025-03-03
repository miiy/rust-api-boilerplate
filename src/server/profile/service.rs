use super::error::ProfileError;
use crate::server::user::service as user_service;
use sqlx::MySqlPool;
use time::OffsetDateTime;

// profile
#[derive(Debug)]
pub struct ProfileRequest {
    pub username: String,
}

#[derive(Debug)]
pub struct ProfileResponse {
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub current_status: String,
    pub bio: String,
    pub created_at: Option<OffsetDateTime>,
}

pub async fn profile(
    req: ProfileRequest,
    pool: &MySqlPool,
) -> Result<ProfileResponse, ProfileError> {
    let user = user_service::get_user_by_username(&req.username, &pool)
        .await
        .map_err(|e| ProfileError::Service(e.to_string()))?;
    let user_profile = user_service::get_user_profile(user.id, &pool)
        .await
        .map_err(|e| ProfileError::Service(e.to_string()))?;

    let resp = ProfileResponse {
        username: user.username,
        nickname: user_profile.nickname,
        avatar: user_profile.avatar,
        current_status: user_profile.current_status,
        bio: user_profile.bio,
        created_at: user_profile.created_at,
    };
    Ok(resp)
}
