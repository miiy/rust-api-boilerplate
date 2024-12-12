pub mod config;
pub mod db;
pub mod error;
pub mod json_config;
pub mod jwt;
pub mod middleware;
pub mod pagination;
pub mod server;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub jwt: jwt::JWT,
}

impl AppState {
    pub fn new(c: &config::Config) -> Self {
        // db
        let pool = db::init_pool(&c.database.url).expect("Failed to create pool");

        // redis
        let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

        // jwt
        let jwt = jwt::JWT::new(c.jwt.secret.clone(), c.jwt.expires_in);
        Self {
            db: pool,
            redis,
            jwt,
        }
    }
}
