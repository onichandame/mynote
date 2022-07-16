use async_graphql::async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::auth::Session;

pub struct NoteHook {}

#[async_trait]
impl crud::Hook for NoteHook {
    type ActiveModel = entity::note::ActiveModel;
    async fn before_create(
        &self,
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        input.user_id = sea_orm::Set(user.id);
        input.created_at = sea_orm::Set(chrono::Utc::now().naive_utc());
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now().naive_utc()));
        Ok(input)
    }
}
