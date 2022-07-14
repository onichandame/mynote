use async_graphql::{async_trait::async_trait, Context, Guard, Result};

use crate::auth::Session;

#[derive(Default)]
pub struct LoggedIn {}

#[async_trait]
impl Guard for LoggedIn {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        ctx.data::<Session>()
            .map(|_| ())
            .map_err(|_| "not logged in".into())
    }
}
