pub use sea_orm_migration::prelude::*;

mod m20220711_000001_create_table_user;
mod m20220711_000002_create_table_credential;
mod m20220711_000003_create_table_memo;
mod m20220714_000001_create_table_session_key;
mod m20221009_000001_update_table_memo;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220711_000001_create_table_user::Migration),
            Box::new(m20220711_000002_create_table_credential::Migration),
            Box::new(m20220711_000003_create_table_memo::Migration),
            Box::new(m20220714_000001_create_table_session_key::Migration),
            Box::new(m20221009_000001_update_table_memo::Migration),
        ]
    }
}
