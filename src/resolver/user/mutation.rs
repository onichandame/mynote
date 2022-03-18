use async_graphql::{Context, Object, Result};
use user::UserModule;

use crate::{
    conversion::IntoUniversal,
    dto::{UserDTO, UserUpdateDTO},
    get_user,
};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn update_user(&self, ctx: &Context<'_>, update: UserUpdateDTO) -> Result<UserDTO> {
        let user_module = ctx.data::<UserModule>()?;
        get_user!(user, ctx);
        Ok(UserDTO::from(
            &user_module
                .update(
                    user.id,
                    update.name,
                    None,
                    update.email.into_universal(),
                    update.avatar.into_universal(),
                )
                .await?,
        ))
    }
}
