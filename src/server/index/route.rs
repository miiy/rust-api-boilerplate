use super::handler;
use actix_web::{web, Scope};

// GET /
pub fn routes() -> Scope {
    web::scope("/").service(web::resource("").route(web::get().to(handler::index)))
}
