use std::error::Error;

use config::ConfigProvider;
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::{migrate, sqlite};

/// constructor
pub async fn new_db_connection(
    config: ConfigProvider,
) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    let pool = sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    migrate!().run(&pool).await?;
    Ok(SqlxSqliteConnector::from_sqlx_sqlite_pool(pool))
}

#[cfg(test)]
mod tests {
    use config::{new_config_provider, Mode};

    use super::*;

    #[tokio::test]
    async fn connection_pool() -> Result<(), Box<dyn Error + Send + Sync>> {
        init().await?;
        Ok(())
    }

    async fn init() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
        let config = new_config_provider(Mode::UnitTest)?;
        Ok(new_db_connection(config.clone()).await?)
    }
}
