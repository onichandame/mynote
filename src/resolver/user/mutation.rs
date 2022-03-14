use async_graphql::{Context, Object, Result};
use session::SessionModule;
use user::UserModule;

use crate::{
    conversion::IntoUniversal,
    dto::{UserDTO, UserUpdateDTO},
    session::Session,
};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let token = ctx.data::<Session>()?;
        let session = ctx.data::<SessionModule>()?;
        let user_module = ctx.data::<UserModule>()?;
        let user = session.deserialize(token).await?;
        Ok(UserDTO::from(
            &user_module
                .update(
                    user.id,
                    update.name,
                    update.password,
                    update.email.into_universal(),
                    update.avatar.into_universal(),
                )
                .await?,
        ))
    }
}
