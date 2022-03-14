use async_graphql::{Context, Object, Result};
use session::SessionModule;

use crate::{dto::UserDTO, guard::LoginRequired, session::Session};

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    #[graphql(guard = "LoginRequired::new()")]
    async fn me(&self, ctx: &Context<'_>) -> Result<UserDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        Ok(UserDTO::from(&session.deserialize(token).await?))
    }
}
