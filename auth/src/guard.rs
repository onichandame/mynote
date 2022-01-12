use crate::service::get_user_from_ctx;

pub struct Auth;

#[derive(Default)]
pub struct LoginRequired;

#[async_trait::async_trait]
impl async_graphql::Guard for LoginRequired {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        Ok(get_user_from_ctx(ctx).await.map(|_| ())?)
    }
}
impl LoginRequired {
    pub fn new() -> Self {
        Self
    }
}
