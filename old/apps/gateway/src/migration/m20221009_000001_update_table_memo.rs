use sea_orm_migration::prelude::*;

use super::tables::Memo;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221009_000001_update_table_memo"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Memo::Table)
                    .add_column_if_not_exists(ColumnDef::new(Memo::Weight).integer())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .drop_column(Memo::Weight)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
