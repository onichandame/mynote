use async_graphql::{Context, InputObject, Object, Result};

use crate::{entity, Gateway};

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
    pub invitation_key: Option<String>,
}

#[derive(InputObject)]
struct ChangePasswordInput {
    pub password: String,
}

#[Object]
impl AuthMutation {
    async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String> {
        ctx.data::<entity::user::Model>()
            .err()
            .ok_or("logged in user cannot signup again")?;
        let nb = ctx.data::<Gateway>()?;
        let user = nb
            .auth
            .signup(
                &input.name,
                &input.password,
                input.email.as_deref(),
                input.invitation_key.as_deref(),
            )
            .await?;
        Ok(nb.auth.session.generate_for_user(&user).await?.token)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        ctx.data::<entity::user::Model>()
            .err()
            .ok_or("logged in user cannot log in again")?;
        let nb = ctx.data::<Gateway>()?;
        let session = nb
            .auth
            .login_by_password(&input.identity, &input.password)
            .await?;
        Ok(session.token)
    }

    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let user = ctx.data::<entity::user::Model>()?;
        let nb = ctx.data::<Gateway>()?;
        Ok(nb.auth.session.generate_for_user(&user).await?.token)
    }

    async fn change_password(&self, ctx: &Context<'_>, input: ChangePasswordInput) -> Result<bool> {
        let nb = ctx.data::<Gateway>()?;
        let user = ctx.data::<entity::user::Model>()?;
        nb.auth.credential.update(user.id, &input.password).await?;
        Ok(true)
    }
}
