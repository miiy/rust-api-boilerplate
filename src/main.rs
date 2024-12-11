use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_api::{config, json_config, middleware, server, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    let addrs = c.server.addrs.clone();
    HttpServer::new(move || {
        let app_state = AppState::new(&c);
        let shared_data = web::Data::new(app_state);
        App::new()
            .wrap(middleware::cors::cors(&c.app.url))
            .wrap(Logger::default())
            .app_data(shared_data)
            .app_data(json_config::json_config())
            .configure(server::route::config_api)
    })
    .bind(addrs)?
    .run()
    .await
}
