use anyhow::Context;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::{self, prelude::*};

use super::{credential, session::Session};

pub async fn login_by_password(
    identity: &str,
    password: &str,
    db: &DatabaseConnection,
) -> anyhow::Result<Session> {
    let user = User::find()
        .filter(
            Condition::any()
                .add(entity::user::Column::Email.eq(identity.clone()))
                .add(entity::user::Column::Name.eq(identity.clone())),
        )
        .one(db)
        .await?
        .context("user not found")?;
    if !credential::check_credential(user.id, password, db).await? {
        anyhow::bail!("password incorrect");
    }
    Ok(Session::try_from_user(&user, db).await?)
}
