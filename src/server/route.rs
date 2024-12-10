use super::auth;
use super::index;
use super::post;
use super::profile;
use super::user;
use crate::middleware::authentication;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    let auth_middleware = authentication::Authentication;

    user::route::init_routes(cfg);
    post::route::init_routes(cfg);
    profile::route::init_routes(cfg, auth_middleware);
    auth::route::init_routes(cfg);
    index::route::init_routes(cfg);
}