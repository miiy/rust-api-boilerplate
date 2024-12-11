use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub fn init_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect_lazy(database_url)
}
