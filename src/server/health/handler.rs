use actix_web::{HttpResponse, Result};

// GET /
pub async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("OK"))
}
