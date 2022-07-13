use async_graphql::{Context, InputObject, Object, Result};

#[derive(Default)]
pub struct AuthMutation {}

#[derive(InputObject)]
struct LoginInput {
    pub identity: String,
    pub password: String,
}

#[Object]
impl AuthMutation {
    #[graphql(guard = "super::super::guards::NotLoggedIn::default()")]
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        todo!()
    }
    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        todo!()
    }
}
