use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{
    auth::{login_by_password, signup, Session},
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
        ctx.data::<Session>()
            .err()
            .ok_or("logged in user cannot signup again")?;
        let db = ctx.data::<DatabaseConnection>()?;
        let user = signup(&input.name, &input.password, input.email.as_deref(), db).await?;
        Ok(Session::encode(&user, db).await?.0)
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = login_by_password(&input.identity, &input.password, db).await?;
        Ok(session.0)
    }

    async fn renew_session(&self, ctx: &Context<'_>) -> Result<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
        Ok(Session::encode(&user, &db).await?.0)
    }

    async fn change_password(&self, ctx: &Context<'_>, input: ChangePasswordInput) -> Result<bool> {
        let db = ctx.data::<DatabaseConnection>()?;
        let session = ctx.data::<Session>()?;
        let user = session.decode(db).await?;
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
