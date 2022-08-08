use sea_orm_migration::prelude::*;

use super::tables::SessionKey;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220714_000001_create_table_session_key"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(SessionKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SessionKey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SessionKey::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(SessionKey::Key).text().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("session_key-created_at")
                    .table(SessionKey::Table)
                    .col((SessionKey::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(SessionKey::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
