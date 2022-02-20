use migration::run_migration;
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::sqlite;
use std::{env, error::Error};

mod migration;

pub async fn new_database_connection() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    Ok(SqlxSqliteConnector::from_sqlx_sqlite_pool(
        new_connection_pool().await?,
    ))
}

async fn new_connection_pool() -> Result<sqlite::SqlitePool, Box<dyn Error + Send + Sync>> {
    let db_addr_key = "DATABASE_URL";
    let db_addr = match env::var("UNITTEST") {
        Ok(_) => String::from("sqlite://:memory:"),
        _other => env::var(&db_addr_key).expect("DATABASE_URL not set"),
    };
    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_addr)
        .await?;
    run_migration(&pool).await?;
    Ok(pool)
}
