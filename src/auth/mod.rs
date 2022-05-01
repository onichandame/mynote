use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use session::SessionModule;

use crate::helper::get_user_from_ctx;

#[derive(Default)]
pub struct AuthQuery {}
#[derive(Default)]
pub struct AuthMutation {}

#[derive(InputObject)]
pub struct LoginInput {
    pub identity: String,
    pub password: String,
}

#[Object]
impl AuthMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let session_module = ctx.data::<SessionModule>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        let user = model::user::Entity::find()
            .filter(
                sea_orm::Condition::any()
                    .add(model::user::Column::Email.eq(input.identity.clone()))
                    .add(model::user::Column::Name.eq(input.identity.clone())),
            )
            .one(db)
            .await?
            .ok_or("user not found")?;
        if user.check_password(&input.password)? {
            Ok(session_module.serialize(&user).await?)
        } else {
            Err("password incorrect".into())
        }
    }
    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let user = get_user_from_ctx(ctx).await?;
        let session_module = ctx.data::<SessionModule>()?;
        Ok(session_module.serialize(&user).await?)
    }
}

#[Object]
impl AuthQuery {
    async fn validate_password(&self, ctx: &Context<'_>, password: String) -> Result<bool> {
        let user = get_user_from_ctx(ctx).await?;
        Ok(user.check_password(&password)?)
    }
}
