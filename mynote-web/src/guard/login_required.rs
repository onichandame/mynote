use mynote_core::MyNote;

use crate::session::Session;

#[derive(Default)]
pub struct LoginRequired;

#[async_trait::async_trait]
impl async_graphql::Guard for LoginRequired {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let session = ctx.data::<Session>()?;
        let core = ctx.data::<MyNote>()?;
        Ok(core.auth.get_user_for_session(session).await.map(|_| ())?)
    }
}

impl LoginRequired {
    pub fn new() -> Self {
        Self
    }
}
