use sea_orm_migration::prelude::*;

use super::tables::Memo;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220711_000003_create_table_memo"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Memo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Memo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Memo::Uuid).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(Memo::Revision)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Memo::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Memo::UpdatedAt).date_time())
                    .col(ColumnDef::new(Memo::DeletedAt).date_time())
                    .col(ColumnDef::new(Memo::Content).text().not_null())
                    .col(
                        ColumnDef::new(Memo::Weight)
                            .unsigned()
                            .default(0)
                            .not_null(),
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
                    .table(Memo::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
