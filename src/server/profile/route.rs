use super::handler;
use actix_web::{web, Scope};

// GET /profile
// PUT /profile
// PUT /profile/password
pub fn routes() -> Scope {
    web::scope("/v1/profile")
        .service(
            web::resource("")
                .route(web::get().to(handler::profile))
                .route(web::put().to(handler::update_profile)),
        )
        .service(
            web::resource("/password")
                .route(web::put().to(handler::change_password)),
        )
}
