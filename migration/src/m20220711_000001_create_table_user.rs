use sea_orm_migration::prelude::*;

use crate::tables::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220711_000001_create_table_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).date_time())
                    .col(ColumnDef::new(User::DeletedAt).date_time())
                    .col(ColumnDef::new(User::Name).text().not_null())
                    .col(ColumnDef::new(User::Email).text())
                    .col(ColumnDef::new(User::Avatar).text())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("user-name")
                    .table(User::Table)
                    .col(User::Name)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(User::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
