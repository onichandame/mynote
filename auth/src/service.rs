use async_graphql;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use model;
use rand::{self, distributions::Alphanumeric, Rng};
use sea_orm::{entity::*, Condition, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::{default::Default, error::Error, ops::Add, str};

use crate::{error::AuthError, Session};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn create_user(
    db: &model::Database,
    input: model::user::ActiveModel,
) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
    Ok(input.insert(db).await?)
}

pub async fn update_user(
    db: &model::Database,
    id: i32,
    update: model::user::ActiveModel,
) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
    Ok(model::User::update(update)
        .filter(model::user::Column::Id.eq(id))
        .exec(db)
        .await?)
}

pub async fn login(
    db: &model::Database,
    name_or_email: &String,
    password: &String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let user = model::User::find()
        .filter(
            Condition::any()
                .add(model::user::Column::Email.eq(name_or_email.to_owned()))
                .add(model::user::Column::Name.eq(name_or_email.to_owned())),
        )
        .one(db)
        .await?
        .ok_or("user not found")?;
    if user.check_password(password)? {
        Ok(create_session(db, user.id).await)
    } else {
        Err(Box::new(AuthError::IncorrectPassword))
    }
}

pub async fn create_session(db: &model::Database, uid: i32) -> String {
    let claims = Claims {
        sub: uid.to_string(),
        exp: usize::try_from(Utc::now().add(Duration::days(31)).naive_utc().timestamp()).unwrap(),
    };
    let key = get_latest_key(db).await.unwrap();
    jsonwebtoken::encode(
        &Header {
            kid: Some(key.id.to_string()),
            ..Default::default()
        },
        &claims,
        &EncodingKey::from_secret(key.key.as_ref()),
    )
    .unwrap()
}

pub async fn get_user_from_ctx(
    ctx: &async_graphql::Context<'_>,
) -> async_graphql::Result<model::user::Model> {
    let db = ctx.data::<model::Database>().unwrap();
    let session = ctx
        .data::<Session>()
        .map_err(|_| async_graphql::Error::new("Unauthorized"))?;
    Ok(deserialize_session(db, session).await?)
}

pub async fn deserialize_session(
    db: &model::Database,
    session: &str,
) -> Result<model::user::Model, Box<dyn Error + Sync + Send>> {
    let meta = jsonwebtoken::decode_header(session)?;
    let kid = meta.kid.ok_or("session key id empty")?;
    let key = model::SessionKey::find_by_id(kid.parse()?)
        .one(db)
        .await?
        .ok_or("session key not found")?;
    let claim = jsonwebtoken::decode::<Claims>(
        session,
        &DecodingKey::from_secret(&key.key.as_ref()),
        &Validation::default(),
    )?
    .claims;
    let uid = claim.sub;
    Ok(model::User::find_by_id(uid.parse()?)
        .one(db)
        .await?
        .ok_or("user not found")?)
}

async fn get_latest_key(
    db: &model::Database,
) -> Result<model::session_key::Model, Box<dyn Error + Sync + Send>> {
    let key_or_err = model::SessionKey::find()
        .order_by_desc(model::session_key::Column::CreatedAt)
        .one(db)
        .await?
        .ok_or(AuthError::SessionKeyNotFound);
    let create_key = || async {
        let key = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();
        Ok(model::session_key::ActiveModel {
            key: Set(key),
            ..Default::default()
        }
        .insert(db)
        .await?)
    };
    match key_or_err {
        Ok(key) => {
            if chrono::Utc::now()
                .naive_utc()
                .signed_duration_since(key.created_at)
                .gt(&chrono::Duration::days(30))
            {
                create_key().await
            } else {
                Ok(key)
            }
        }
        Err(e) => match e {
            AuthError::SessionKeyNotFound => create_key().await,
            other => Err(other.into()),
        },
    }
}
