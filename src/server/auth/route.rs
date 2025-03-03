use super::handler;
use actix_web::{web, Scope};

// POST /auth/register
// POST /auth/login
pub fn routes() -> Scope {
    web::scope("/v1/auth")
        .service(web::resource("/register").route(web::post().to(handler::register)))
        .service(web::resource("/login").route(web::post().to(handler::login)))
        .service(web::resource("/logout").route(web::post().to(handler::logout)))
}
