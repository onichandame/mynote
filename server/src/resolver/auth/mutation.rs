use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{
    auth::{login::login_by_password, session::Session, signup::signup},
    entity,
};

#[derive(Default)]
pub struct AuthMutation {}

#[derive(InputObject)]
struct LoginInput {
    pub identity: String,
    pub password: String,
}

#[derive(InputObject)]
struct SignupInput {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(InputObject)]
struct ChangePasswordInput {
    pub password: String,
}

#[Object]
impl AuthMutation {
    async fn signup(&self, ctx: &Context<'_>, input: SignupInput) -> Result<String> {
        ctx.data::<entity::user::Model>()
            .err()
            .ok_or("logged in user cannot signup again")?;
        let db = ctx.data::<DatabaseConnection>()?;
        let user = signup(&input.name, &input.password, input.email.as_deref(), db).await?;
        Ok(Session::try_from_user(&user, db).await?.token)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        ctx.data::<entity::user::Model>()
            .err()
            .ok_or("logged in user cannot log in again")?;
        let db = ctx.data::<DatabaseConnection>()?;
        let session = login_by_password(&input.identity, &input.password, db).await?;
        Ok(session.token)
    }

    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let user = ctx.data::<entity::user::Model>()?;
        let db = ctx.data::<DatabaseConnection>()?;
        Ok(Session::try_from_user(&user, &db).await?.token)
    }

    async fn change_password(&self, ctx: &Context<'_>, input: ChangePasswordInput) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<entity::user::Model>()?;
        entity::credential::ActiveModel {
            password: Set(input.password),
            user_id: Set(user.id),
            ..Default::default()
        }
        .insert(db)
        .await?;
        Ok(true)
    }
}
