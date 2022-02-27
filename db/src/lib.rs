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
    let db_addr = env::var(&db_addr_key)?;
    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_addr)
        .await?;
    run_migration(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn connection_pool() -> Result<(), Box<dyn Error + Send + Sync>> {
        assert!(new_connection_pool().await.is_err());
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        assert!(new_connection_pool().await.is_ok());
        Ok(())
    }
}
