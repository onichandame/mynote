use async_graphql::{Context, Object, Result};
use auth::{AuthModule, LoginRequired};
use dto::UserDTO;

use crate::{
    dto::{IntoActiveModel, LoginInputDTO, UserCreateDTO, UserUpdateDTO},
    user::UserModule,
};

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct UserMutation;
#[derive(Default)]
pub struct SessionMutation;

#[Object]
impl UserQuery {
    #[graphql(name = "self", guard = "LoginRequired::new()")]
    async fn get_user<'a>(&self, ctx: &Context<'a>) -> Result<UserDTO> {
        let auth_module = ctx.data::<AuthModule>()?;
        Ok(UserDTO::from(&auth_module.get_user_from_ctx(ctx).await?))
    }
}

#[Object]
impl UserMutation {
    async fn sign_up(&self, ctx: &Context<'_>, input: UserCreateDTO) -> Result<UserDTO> {
        let user_module = ctx.data::<UserModule>()?;
        Ok(UserDTO::from(
            &user_module.create_user(input.into_active_model()).await?,
        ))
    }
    #[graphql(guard = "LoginRequired::new()")]
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let auth_module = ctx.data::<AuthModule>()?;
        let user_module = ctx.data::<UserModule>()?;
        let user = auth_module.get_user_from_ctx(ctx).await?;
        Ok(UserDTO::from(
            &user_module
                .update_user(user.id, update.into_active_model())
                .await?,
        ))
    }
}

#[Object]
impl SessionMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInputDTO) -> Result<String> {
        let db = ctx.data::<db::DatabaseConnection>().unwrap();
        Ok(service::login(db, &input.name, &input.password).await?)
    }
}
