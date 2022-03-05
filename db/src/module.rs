use std::error::Error;

use config::ConfigModule;
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::{migrate, sqlite};

pub struct DbModule {}

/// constructor
impl DbModule {
    pub async fn create(
        config: &ConfigModule,
    ) -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
        let pool = sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
        migrate!().run(&pool).await?;
        Ok(SqlxSqliteConnector::from_sqlx_sqlite_pool(pool))
    }
}

#[cfg(test)]
mod tests {
    use config::{ConfigModule, Mode};

    use super::*;

    #[tokio::test]
    async fn connection_pool() -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = ConfigModule::create(Mode::UnitTest)?;
        DbModule::create(&config).await?;
        Ok(())
    }
}
