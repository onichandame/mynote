use sea_orm::ActiveModelTrait;

#[async_trait::async_trait]
pub trait Hook {
    type ActiveModel: ActiveModelTrait + Send;
    async fn before_create(
        &self,
        _ctx: &async_graphql::Context<'_>,
        input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        Ok(input)
    }
    async fn before_update(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        Ok(input)
    }
    async fn before_delete(
        &self,
        _ctx: &async_graphql::Context<'_>,
        _filter: sea_orm::Condition,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<()> {
        Ok(())
    }
}
