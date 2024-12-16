use super::dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use super::error::AuthError;
use super::model::{User, UserStatus};
use super::password;
use crate::jwt::{AuthenticatedUser, JWT};
use sqlx::MySqlPool;
use time::OffsetDateTime;

pub struct Service;

impl Service {
    pub async fn register(
        req: RegisterRequest,
        pool: &MySqlPool,
    ) -> Result<RegisterResponse, AuthError> {
        Self::validate_register(&req)?;

        if !User::check_username_available(pool, &req.username).await? {
            return Err(AuthError::UsernameAlreadyExists);
        }

        if !User::check_email_available(pool, &req.email).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        let hashed = password::bcrypt_hash(&req.password)?;
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
        Self::validate_login(&req)?;

        let user_potion = User::find_by_username(&pool, req.username).await?;
        if let Some(user) = user_potion {
            if !password::bcrypt_verify(&req.password, &user.password)? {
                return Err(AuthError::WrongPassword);
            }

            let claims = jwt.create_claims(user.username.clone());
            let token = jwt.encode(&claims)?;

            return Ok(LoginResponse {
                token_type: "Bearer".to_string(),
                access_token: token,
                expires_in: jwt.expires_in,
                user: AuthenticatedUser { username: user.username },
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
}
