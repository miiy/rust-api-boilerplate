use super::handler;
use actix_web::{web, Scope};

// GET /health
pub fn routes() -> Scope {
    web::scope("/health").service(web::resource("").route(web::get().to(handler::index)))
}
