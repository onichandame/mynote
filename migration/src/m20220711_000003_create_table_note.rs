use sea_orm_migration::prelude::*;

use crate::tables::{Note, User};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220711_000003_create_table_note"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Note::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Note::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Note::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Note::UpdatedAt).date_time())
                    .col(ColumnDef::new(Note::DeletedAt).date_time())
                    .col(ColumnDef::new(Note::UserId).integer().not_null())
                    .col(ColumnDef::new(Note::Title).text().not_null())
                    .col(ColumnDef::new(Note::Content).text().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .from(Note::Table, Note::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("note-userId_deletedAt_updatedAt")
                    .table(Note::Table)
                    .col(Note::UserId)
                    .col(Note::DeletedAt)
                    .col(Note::UpdatedAt)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Note::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
