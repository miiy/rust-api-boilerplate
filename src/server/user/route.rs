use super::handler;
use actix_web::{web, Scope};

// GET /users/{username}
pub fn routes() -> Scope {
    web::scope("/v1/users")
        .service(
            web::scope("/{username}")
                .service(web::resource("").route(web::get().to(handler::user)))
                .service(web::resource("/activity").route(web::get().to(handler::activity)))
                .service(web::resource("/followers").route(web::get().to(handler::followers)))
                .service(web::resource("/following").route(web::get().to(handler::following)))
    )
}
