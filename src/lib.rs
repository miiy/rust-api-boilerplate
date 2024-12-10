use crate::error::{ErrorEntity, ErrorResponse};
use crate::jwt::JWT;
use actix_web::{error::JsonPayloadError, web::JsonConfig, HttpRequest, HttpResponse};

pub mod config;
pub mod db;
pub mod error;
pub mod jwt;
pub mod middleware;
pub mod pagination;
pub mod server;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub jwt: JWT,
}

// Actix Web Json Config
pub fn json_config() -> JsonConfig {
    JsonConfig::default().error_handler(json_error_handler)
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::error::Error {
    let error_response = ErrorResponse {
        error: ErrorEntity {
            code: 400,
            message: err.to_string(),
        },
    };
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(error_response),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(error_response)
        }
        _ => HttpResponse::BadRequest().json(error_response),
    };
    actix_web::error::InternalError::from_response(err, resp).into()
}
