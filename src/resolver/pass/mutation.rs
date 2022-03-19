use async_graphql::{Context, Object, Result};
use pass::PassModule;

use crate::{
    dto::{PassDTO, PassInputDTO},
    get_user,
};

#[derive(Default)]
pub struct PassMutation {}

#[Object]
impl PassMutation {
    #[graphql("guard=LoginRequired::new()")]
    async fn create_password(&self, ctx: &Context<'_>, input: PassInputDTO) -> Result<PassDTO> {
        get_user!(user, ctx);
        let pass = ctx.data::<PassModule>()?;
        Ok((&pass.create(user.id, &input.name, &input.password).await?).into())
    }
}
