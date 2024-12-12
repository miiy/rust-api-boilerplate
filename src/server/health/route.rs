use super::handler;
use actix_web::web;

// GET /
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health").service(web::resource("").route(web::get().to(handler::index))),
    );
}
