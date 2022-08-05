use entity::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, str::FromStr};

use async_graphql::Result;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryOrder, Set,
    TransactionTrait,
};

pub struct Session(pub String);

#[derive(Serialize, Deserialize)]
struct Claims {
    /// user id
    sub: String,
    /// expiration date in seconds after epoch
    exp: usize,
}

impl FromStr for Session {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl Session {
    pub async fn decode(&self, db: &DatabaseConnection) -> Result<entity::user::Model> {
        let meta = jsonwebtoken::decode_header(&self.0)?;
        let kid = meta.kid.ok_or("session key id empty")?;
        let key_doc = SessionKey::find_by_id(kid.parse()?)
            .one(db)
            .await?
            .ok_or(format!("session key {} not found", kid))?;
        let claims = jsonwebtoken::decode::<Claims>(
            &self.0,
            &DecodingKey::from_secret(key_doc.key.as_bytes()),
            &Validation::default(),
        )?
        .claims;
        if chrono::Utc::now().naive_utc()
            < chrono::NaiveDateTime::from_timestamp(claims.exp.try_into()?, 0)
        {
            User::find_by_id(claims.sub.parse()?)
                .one(db)
                .await?
                .ok_or("user not found".into())
        } else {
            Err("session expired".into())
        }
    }
    pub async fn encode(user: &entity::user::Model, db: &DatabaseConnection) -> Result<Self> {
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::days(7))
                .naive_utc()
                .timestamp()
                .try_into()?,
        };
        let key_doc = get_active_key(db).await?;
        let token = jsonwebtoken::encode(
            &Header {
                kid: Some(key_doc.id.to_string()),
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_secret(key_doc.key.as_bytes()),
        )?;
        Ok(Self(token))
    }
}

async fn get_active_key(db: &DatabaseConnection) -> Result<entity::session_key::Model> {
    async fn create_key<TDb: ConnectionTrait>(txn: &TDb) -> Result<entity::session_key::Model> {
        let key = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();
        Ok(entity::session_key::ActiveModel {
            created_at: Set(chrono::Utc::now().naive_utc()),
            key: Set(key),
            ..Default::default()
        }
        .insert(txn)
        .await?)
    }
    let key_doc = SessionKey::find()
        .order_by_desc(entity::session_key::Column::CreatedAt)
        .one(db)
        .await?;
    match key_doc {
        Some(key_doc) => {
            let key_doc = if chrono::Utc::now().naive_utc()
                > key_doc.created_at + chrono::Duration::days(31)
            {
                let txn = db.begin().await?;
                let key_doc = create_key(&txn).await?;
                txn.commit().await?;
                key_doc
            } else {
                key_doc
            };
            Ok(key_doc)
        }
        None => create_key(db).await,
    }
}
