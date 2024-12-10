use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_api::{config, db, json_config, jwt, middleware, server, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // db
    let pool = db::init_pool(&c.database.url)
        .await
        .expect("Failed to create pool");

    // redis
    let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

    // jwt
    let jwt = jwt::JWT::new(c.jwt.secret.clone(), c.jwt.expires_in);

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    HttpServer::new(move || {
        let shared_data = web::Data::new(AppState {
            db: pool.clone(),
            redis: redis.clone(),
            jwt: jwt.clone(),
        });

        App::new()
            .wrap(middleware::cors::cors(&c.app.url))
            .wrap(Logger::default())
            .app_data(shared_data)
            .app_data(json_config())
            .configure(server::route::config_api)
    })
    .bind(&c.server.addrs)?
    .run()
    .await
}
