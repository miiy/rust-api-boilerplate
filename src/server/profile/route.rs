use super::handler;
use crate::middleware::authentication;
use actix_web::web;

// GET /profile
pub fn init_routes(cfg: &mut web::ServiceConfig, auth_middleware: authentication::Authentication) {
    cfg.service(
        web::scope("/v1/profile")
            .wrap(auth_middleware)
            .service(web::resource("").route(web::get().to(handler::profile))),
    );
}
