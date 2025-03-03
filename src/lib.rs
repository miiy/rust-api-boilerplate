pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod json_config;
pub mod middleware;
pub mod pagination;
pub mod server;
pub mod datetime;
use crate::auth::provider_default::DefaultAuthenticationProvider;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub jwt: auth::jwt::JWT,
    pub auth_provider: DefaultAuthenticationProvider,
}

impl AppState {
    pub fn new(c: &config::Config) -> Self {
        // db
        let pool = db::init_pool(&c.database.url).expect("Failed to create pool");

        // redis
        let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

        // jwt
        let jwt = auth::jwt::JWT::new(
            c.jwt.secret.clone(),
            c.jwt.expires_in,
            c.jwt.encryption_key.clone(),
        )
        .expect("Failed to create jwt");

        // auth provider
        let auth_provider =
        auth::provider_default::DefaultAuthenticationProvider::new(pool.clone(), redis.clone());

        Self {
            db: pool,
            redis,
            jwt,
            auth_provider,
        }
    }
}
