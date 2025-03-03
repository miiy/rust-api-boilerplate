use super::service;
use crate::error::APIError;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};
// register
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

// login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i64,
}

// POST /auth/register
pub async fn register(
    req: web::Json<RegisterRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let req = service::RegisterRequest {
        username: req.username,
        email: req.email,
        password: req.password,
        password_confirmation: req.password_confirmation,
    };
    let resp = service::register(req, &app_state.db)
        .await
        .map_err(APIError::from)?;
    let resp = RegisterResponse {
        message: resp.message,
    };
    Ok(HttpResponse::Created().json(resp))
}

// POST /auth/login
pub async fn login(
    req: web::Json<LoginRequest>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();
    let req = service::LoginRequest {
        username: req.username,
        password: req.password,
    };
    let resp = service::login(req, &app_state.db, &app_state.jwt)
        .await
        .map_err(APIError::from)?;
    let resp = LoginResponse {
        token_type: resp.token_type,
        access_token: resp.access_token,
        expires_in: resp.expires_in,
    };
    Ok(HttpResponse::Ok().json(resp))
}

// POST /auth/logout
pub async fn logout(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let resp = service::logout(&app_state.db)
        .await
        .map_err(APIError::from)?;
    Ok(HttpResponse::Ok().json(resp))
}
