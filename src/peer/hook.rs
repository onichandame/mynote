use crate::helper::get_user_from_ctx;

pub struct PeerHook {}

#[async_trait::async_trait]
impl crud::Hook for PeerHook {
    type ActiveModel = model::peer::ActiveModel;
    async fn before_create(
        &self,
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let user = get_user_from_ctx(ctx).await?;
        input.user_id = sea_orm::Set(user.id);
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        input.updated_at = sea_orm::Set(Some(chrono::Utc::now()));
        Ok(input)
    }
}
