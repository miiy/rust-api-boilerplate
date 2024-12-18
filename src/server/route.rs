use super::auth;
use super::health;
use super::index;
use super::post;
use super::profile;
use super::user;
use crate::middleware::authentication;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    let auth_middleware = authentication::Authentication;

    // public routes
    index::route::init_routes(cfg);
    health::route::init_routes(cfg);
    auth::route::init_routes(cfg);
    user::route::init_routes(cfg);
    post::route::init_routes(cfg);

    // private routes
    cfg.service(
        web::scope("")
            .wrap(auth_middleware)
            .configure(profile::route::init_routes),
    );
}
