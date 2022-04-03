use async_graphql::{Context, Object, Result};
use auth::AuthModule;
use session::SessionModule;
use user::UserModule;

use crate::{
    dto::{LoginInputDTO, UserCreateDTO, UserDTO},
    get_user,
    guard::LoginRequired,
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
        let user = get_user!(ctx)?;
        Ok(session.serialize(&user).await?)
    }

    #[graphql(guard = "LoginRequired::new()")]
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        old_password: String,
        new_password: String,
    ) -> Result<bool> {
        let user_module = ctx.data::<UserModule>()?;
        let user = get_user!(ctx)?;
        if !user.check_password(&old_password)? {
            return Err("password incorrect".into());
        }
        user_module
            .update(user.id, None, Some(new_password), None, None)
            .await?;
        Ok(true)
    }
}
