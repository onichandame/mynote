use std::error::Error;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryOrder, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::entity::{self, prelude::*};

pub struct Session {
    pub token: String,
    pub user: entity::user::Model,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    /// user id
    sub: String,
    /// expiration date in seconds after epoch
    exp: usize,
}

impl Session {
    pub async fn try_from_token(
        token: &str,
        db: &DatabaseConnection,
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let meta = jsonwebtoken::decode_header(token)?;
        let kid = meta.kid.ok_or("session key id empty")?;
        let key_doc = SessionKey::find_by_id(kid.parse()?)
            .one(db)
            .await?
            .ok_or(format!("session key {} not found", kid))?;
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(key_doc.key.as_bytes()),
            &Validation::default(),
        )?
        .claims;
        if chrono::Utc::now().naive_utc()
            < chrono::NaiveDateTime::from_timestamp(claims.exp.try_into()?, 0)
        {
            Ok(Self {
                token: token.to_owned(),
                user: User::find_by_id(claims.sub.parse()?)
                    .one(db)
                    .await?
                    .ok_or("user not found")?,
            })
        } else {
            Err("session expired".into())
        }
    }
    pub async fn try_from_user(
        user: &entity::user::Model,
        db: &DatabaseConnection,
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
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
        Ok(Self {
            token,
            user: user.to_owned(),
        })
    }
}

async fn get_active_key(
    db: &DatabaseConnection,
) -> Result<entity::session_key::Model, Box<dyn Error + Send + Sync>> {
    async fn create_key<TDb: ConnectionTrait>(
        txn: &TDb,
    ) -> Result<entity::session_key::Model, Box<dyn Error + Send + Sync>> {
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
