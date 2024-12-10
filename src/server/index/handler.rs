use actix_web::{HttpResponse, Result};

// GET /
pub async fn index() -> Result<HttpResponse, actix_web::Error> {
    let app_name = "Rust-API";
    Ok(HttpResponse::Ok().body(format!("Welcome to the {} API!", app_name)))
}
