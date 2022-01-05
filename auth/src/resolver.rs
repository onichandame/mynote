use async_graphql::{Context, Object, Result};
use db::{model, ConnectionPool};

use crate::{
    dto::{LoginInputDTO, UserDTO, UserInputDTO},
    service,
};

#[derive(Default)]
pub struct UserQuery;
#[derive(Default)]
pub struct UserMutation;
#[derive(Default)]
pub struct SessionMutation;

#[Object]
impl UserQuery {
    #[graphql(name = "self")]
    async fn get_user<'a>(&self, ctx: &Context<'a>) -> Result<UserDTO> {
        let res: UserDTO = ctx.data::<model::User>()?.into();
        Ok(res)
    }
}

#[Object]
impl UserMutation {
    async fn register<'a>(&self, ctx: &Context<'a>, input: UserInputDTO) -> Result<UserDTO> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        Ok(UserDTO::from(
            service::create_user(&input.name, &input.password, &input.email, pool).await?,
        ))
    }
}

#[Object]
impl SessionMutation {
    async fn login<'a>(&self, ctx: &Context<'a>, input: LoginInputDTO) -> Result<String> {
        let pool = ctx.data::<ConnectionPool>().unwrap();
        Ok(service::login(&input.name, &input.password, pool).await?)
    }
}
