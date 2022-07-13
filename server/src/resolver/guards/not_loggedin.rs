use async_graphql::{async_trait::async_trait, Context, Guard, Result};

#[derive(Default)]
pub struct NotLoggedIn {}

#[async_trait]
impl Guard for NotLoggedIn {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        Ok(())
    }
}
