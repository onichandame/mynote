use sea_orm_migration::prelude::*;

use super::tables::{Invitation, User};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221018_000001_create_table_invitation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Invitation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Invitation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Invitation::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Invitation::UpdatedAt).date_time())
                    .col(ColumnDef::new(Invitation::InviterId).integer().not_null())
                    .col(
                        ColumnDef::new(Invitation::Key)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Invitation::Table, Invitation::InviterId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("invitation-key")
                    .table(Invitation::Table)
                    .col(Invitation::Key)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Invitation::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
