use async_graphql::{Context, Object, Result};
use auth::AuthModule;
use session::SessionModule;

use crate::{
    dto::{LoginInputDTO, UserCreateDTO, UserDTO},
    guard::LoginRequired,
    session::Session,
};

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn signup(&self, ctx: &Context<'_>, input: UserCreateDTO) -> Result<UserDTO> {
        let auth = ctx.data::<AuthModule>()?;
        Ok(UserDTO::from(
            &auth
                .signup(&input.name, &input.password, input.email, input.avatar)
                .await?,
        ))
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInputDTO) -> Result<String> {
        let session = ctx.data::<SessionModule>()?;
        let auth = ctx.data::<AuthModule>()?;
        Ok(session
            .serialize(
                &auth
                    .login_by_password(&input.identity, &input.password)
                    .await?,
            )
            .await?)
    }

    #[graphql(guard = "LoginRequired::new()")]
    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let session = ctx.data::<SessionModule>()?;
        let token = ctx.data::<Session>()?;
        let user = session.deserialize(token).await?;
        Ok(session.serialize(&user).await?)
    }
}
