use crate::auth::AuthModule;

#[derive(Default)]
pub struct LoginRequired;

#[async_trait::async_trait]
impl async_graphql::Guard for LoginRequired {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let auth_module = ctx.data::<AuthModule>()?;
        Ok(auth_module.get_user_from_ctx(ctx).await.map(|_| ())?)
    }
}

impl LoginRequired {
    pub fn new() -> Self {
        Self
    }
}
