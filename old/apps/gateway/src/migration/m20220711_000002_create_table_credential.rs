use sea_orm_migration::prelude::*;

use super::tables::{Credential, User};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220711_000002_create_table_credential"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Credential::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Credential::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Credential::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Credential::UserId).integer().not_null())
                    .col(ColumnDef::new(Credential::Password).text().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Credential::Table, Credential::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("credential-userId_createdAt")
                    .table(Credential::Table)
                    .col(Credential::UserId)
                    .col((Credential::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Credential::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
