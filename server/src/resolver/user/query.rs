use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

use crate::auth::Session;

use super::dto::UserDTO;

#[derive(Default)]
pub struct UserQuery {}

#[Object]
impl UserQuery {
    #[graphql(guard = "super::super::guards::LoggedIn::default()")]
    async fn get_user(&self, ctx: &Context<'_>) -> Result<UserDTO> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        Ok(user.into())
    }
}
