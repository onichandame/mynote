use db;
use std::env;

pub async fn get_connection_pool() -> db::ConnectionPool {
    env::set_var("UNITTEST", "true");
    db::new_connection_pool().await
}
