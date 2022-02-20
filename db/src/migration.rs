use sqlx::{self, migrate, sqlite};

pub async fn run_migration(db: &sqlite::SqlitePool) -> Result<(), migrate::MigrateError> {
    migrate!().run(db).await
}
