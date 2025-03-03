use super::error::AuthError;
use super::model::{User, UserStatus};
use super::password;
use crate::auth::jwt::JWT;
use sqlx::MySqlPool;
use time::OffsetDateTime;

// register
#[derive(Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug)]
pub struct RegisterResponse {
    pub message: String,
}

// login
#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct LoginResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i64,
}

pub async fn register(
    req: RegisterRequest,
    pool: &MySqlPool,
) -> Result<RegisterResponse, AuthError> {
    validate_register(&req)?;

    if !User::check_username_available(pool, &req.username).await? {
        return Err(AuthError::UsernameAlreadyExists);
    }

    if !User::check_email_available(pool, &req.email).await? {
        return Err(AuthError::EmailAlreadyExists);
    }

    let hashed = password::argon2_hash(&req.password)?;
    let user = User {
        id: 0,
        username: req.username,
        email: req.email,
        phone: "".to_string(),
        password: hashed,
        status: UserStatus::Enabled.as_i8(),
        created_at: Some(OffsetDateTime::now_utc()),
        updated_at: Some(OffsetDateTime::now_utc()),
    };
    User::create(&pool, &user).await?;

    let resp = RegisterResponse {
        message: "Register success".to_string(),
    };
    Ok(resp)
}

fn validate_register(req: &RegisterRequest) -> Result<(), AuthError> {
    if req.username.is_empty()
        || req.email.is_empty()
        || req.password.is_empty()
        || req.password_confirmation.is_empty()
    {
        return Err(AuthError::InvalidArgument("".to_string()));
    }
    if !req.password.eq(&req.password_confirmation) {
        return Err(AuthError::PasswordNotMatch);
    }
    Ok(())
}

pub async fn login(
    req: LoginRequest,
    pool: &MySqlPool,
    jwt: &JWT,
) -> Result<LoginResponse, AuthError> {
    validate_login(&req)?;

    let user_potion = User::find_by_username(&pool, req.username).await?;
    if let Some(user) = user_potion {
        if !password::argon2_verify(&req.password, &user.password)? {
            return Err(AuthError::WrongPassword);
        }

        let claims = jwt.create_claims(user.id.to_string());
        let token = jwt.encode(&claims)?;

        return Ok(LoginResponse {
            token_type: "Bearer".into(),
            access_token: token,
            expires_in: jwt.expires_in,
        });
    }
    Err(AuthError::WrongPassword)
}

fn validate_login(req: &LoginRequest) -> Result<(), AuthError> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(AuthError::InvalidArgument("".to_string()));
    }
    Ok(())
}

pub async fn logout(_pool: &MySqlPool) -> Result<(), AuthError> {
    Ok(())
}