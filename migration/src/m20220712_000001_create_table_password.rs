use sea_orm_migration::prelude::*;

use crate::tables::{Password, User};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220712_000001_create_table_password"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Password::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Password::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Password::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Password::UpdatedAt).date_time())
                    .col(ColumnDef::new(Password::DeletedAt).date_time())
                    .col(ColumnDef::new(Password::UserId).integer().not_null())
                    .col(ColumnDef::new(Password::Title).text().not_null())
                    .col(ColumnDef::new(Password::Password).text().not_null())
                    .col(ColumnDef::new(Password::UserName).text())
                    .col(ColumnDef::new(Password::Icon).text())
                    .col(ColumnDef::new(Password::Url).text())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Password::Table, Password::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Password::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
