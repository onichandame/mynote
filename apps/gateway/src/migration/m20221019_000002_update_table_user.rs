use sea_orm_migration::prelude::*;

use super::tables::{Invitation, User};

static USER_INVITATION_KEY: &str = "FK_user_invitation";

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221019_000002_update_table_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(User::Table)
                    .add_column_if_not_exists(ColumnDef::new(User::InvitationId).integer())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                sea_query::ForeignKey::create()
                    .name(USER_INVITATION_KEY)
                    .from(User::Table, User::InvitationId)
                    .to(Invitation::Table, Invitation::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                sea_query::Index::create()
                    .name("user-invitationId")
                    .table(User::Table)
                    .col(User::InvitationId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                sea_query::Index::drop()
                    .name("user-invitationId")
                    .table(User::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                sea_query::ForeignKey::drop()
                    .table(User::Table)
                    .name(USER_INVITATION_KEY)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                sea_query::Table::alter()
                    .drop_foreign_key(sea_query::Alias::new(USER_INVITATION_KEY))
                    .drop_column(User::InvitationId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
