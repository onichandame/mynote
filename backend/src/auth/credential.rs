use anyhow::Context;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entity::{credential, prelude::*};

pub async fn create_credential<TDb: ConnectionTrait>(
    user_id: i32,
    password: &str,
    db: &TDb,
) -> anyhow::Result<credential::Model> {
    Ok(credential::ActiveModel {
        user_id: Set(user_id),
        password: Set(bcrypt::hash(password, bcrypt::DEFAULT_COST)?),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await?)
}

pub async fn check_credential<TDb: ConnectionTrait>(
    user_id: i32,
    password: &str,
    db: &TDb,
) -> anyhow::Result<bool> {
    let cred = get_active_credential(user_id, db).await?;
    Ok(bcrypt::verify(password, &cred.password)?)
}

pub async fn try_get_active_credential<TDb: ConnectionTrait>(
    user_id: i32,
    db: &TDb,
) -> anyhow::Result<Option<credential::Model>> {
    Ok(Credential::find()
        .filter(credential::Column::UserId.eq(user_id))
        .order_by_desc(credential::Column::Id)
        .one(db)
        .await?)
}

pub async fn get_active_credential<TDb: ConnectionTrait>(
    user_id: i32,
    db: &TDb,
) -> anyhow::Result<credential::Model> {
    Ok(try_get_active_credential(user_id, db)
        .await?
        .with_context(|| "user does not have password set. please contact the admin")?)
}
