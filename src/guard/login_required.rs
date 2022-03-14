use session::SessionModule;

use crate::session::Session;

#[derive(Default)]
pub struct LoginRequired;

#[async_trait::async_trait]
impl async_graphql::Guard for LoginRequired {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        Ok(session.deserialize(token).await.map(|_| ())?)
    }
}

impl LoginRequired {
    pub fn new() -> Self {
        Self
    }
}
