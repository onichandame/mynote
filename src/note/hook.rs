use crate::helper::get_user_from_ctx;

pub struct NoteHook {}

#[async_trait::async_trait]
impl crud::Hook for NoteHook {
    type ActiveModel = model::note::ActiveModel;
    async fn before_create(
        &self,
        ctx: &async_graphql::Context<'_>,
        mut input: Self::ActiveModel,
        _txn: &sea_orm::DatabaseTransaction,
    ) -> async_graphql::Result<Self::ActiveModel> {
        let user = get_user_from_ctx(ctx).await?;
        input.user_id = sea_orm::Set(user.id);
        Ok(input)
    }
}
