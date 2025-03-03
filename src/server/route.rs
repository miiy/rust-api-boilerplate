use super::article;
use super::auth;
use super::health;
use super::index;
use super::user;
use super::profile;
use crate::middleware::authentication;
use actix_web::web;

pub fn config_api(cfg: &mut web::ServiceConfig) {
    let auth_mw = authentication::Authentication;

    // public
    cfg.service(index::route::routes());
    cfg.service(health::route::routes());
    cfg.service(auth::route::routes());

    // jwt auth
    cfg.service(
        web::scope("")
            .wrap(auth_mw)
            .service(user::route::routes())
            .service(profile::route::routes())
            .service(article::route::routes())
    );

}
