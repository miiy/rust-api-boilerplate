use super::handler;
use actix_web::web;

// GET /profile
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/profile").service(web::resource("").route(web::get().to(handler::profile))),
    );
}
