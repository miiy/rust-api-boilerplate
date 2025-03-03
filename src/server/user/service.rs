use super::error::UserError;
use super::model::{User, UserProfile};
use sqlx::MySqlPool;
use time::OffsetDateTime;


#[derive(Debug)]
#[allow(dead_code)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub status: i8,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[allow(dead_code)]
pub enum UserStatus {
    Active = 1,
    Inactive = 0,
}

pub async fn get_user(id: u64, pool: &MySqlPool) -> Result<UserResponse, UserError> {
    let user_option = User::find(&pool, id).await?;
    if let Some(user) = user_option {
        let resp = UserResponse {
            id: user.id,
            username: user.username.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            phone: user.phone.unwrap_or_default(),
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };
        Ok(resp)
    } else {
        Err(UserError::NotFound)
    }
}

pub async fn get_user_by_username(username: &str, pool: &MySqlPool) -> Result<UserResponse, UserError> {
    let user_option = User::find_by_username(&pool, username).await?;

    if let Some(user) = user_option {
        let resp = UserResponse {
            id: user.id,
            username: user.username.unwrap_or_default(),
            email: user.email.unwrap_or_default(),
            phone: user.phone.unwrap_or_default(),
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };
        Ok(resp)
    } else {
        Err(UserError::NotFound)
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct GetUserProfileResponse {
    pub id: u64,
    pub nickname: String,
    pub avatar: String,
    pub current_status: String,
    pub bio: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

pub async fn get_user_profile(user_id: u64, pool: &MySqlPool) -> Result<GetUserProfileResponse, UserError> {
    let user_profile_option = UserProfile::get_user_profile(&pool, user_id).await?;
    if let Some(user_profile) = user_profile_option {
        let resp = GetUserProfileResponse {
            id: user_profile.id,
            nickname: user_profile.nickname,
            avatar: user_profile.avatar,
            current_status: user_profile.current_status,
            bio: user_profile.bio,
            created_at: user_profile.created_at,
            updated_at: user_profile.updated_at,
        };
        Ok(resp)
    } else {
        Err(UserError::NotFound)
    }
}
