use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::DatabaseConnection;

use crate::auth::{login_by_password, signup, Session};

#[derive(Default)]
pub struct AuthMutation {}

#[derive(InputObject)]
struct LoginInput {
    pub identity: String,
    pub password: String,
}

#[derive(InputObject)]
struct SignupInput {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

#[Object]
impl AuthMutation {
    async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String> {
        ctx.data::<Session>()
            .err()
            .ok_or("logged in user cannot signup again")?;
        let db = ctx.data::<DatabaseConnection>()?;
        let user = signup(&input.name, &input.password, input.email.as_deref(), db).await?;
        Ok(Session::encode(&user, db).await?.0)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = login_by_password(&input.identity, &input.password, db).await?;
        Ok(session.0)
    }

    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        Ok(Session::encode(&user, &db).await?.0)
    }
}
