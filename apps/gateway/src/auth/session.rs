use anyhow::Context;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::entity::{self, prelude::*};

use super::credential::CredentialModule;

#[derive(Clone)]
pub struct SessionModule {
    credential: CredentialModule,
    db: DatabaseConnection,
}

pub struct Session {
    pub token: String,
    pub user: entity::user::Model,
    pub credential_id: i32,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    /// user id
    sub: String,
    /// credential id
    cid: i32,
    /// expiration date in seconds after epoch
    exp: usize,
}

impl SessionModule {
    pub fn new(credential: CredentialModule, db: DatabaseConnection) -> Self {
        Self { credential, db }
    }

    async fn get_active_key(&self) -> anyhow::Result<entity::session_key::Model> {
        async fn create_key<TDb: ConnectionTrait>(
            txn: &TDb,
        ) -> anyhow::Result<entity::session_key::Model> {
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
            .one(&self.db)
            .await?;
        match key_doc {
            Some(key_doc) => {
                let key_doc = if chrono::Utc::now().naive_utc()
                    > key_doc.created_at + chrono::Duration::days(31)
                {
                    let key_doc = create_key(&self.db).await?;
                    key_doc
                } else {
                    key_doc
                };
                Ok(key_doc)
            }
            None => create_key(&self.db).await,
        }
    }

    pub async fn generate_for_user(&self, user: &entity::user::Model) -> anyhow::Result<Session> {
        let active_credential = self.credential.get_active_credential(user.id).await?;
        let claims = Claims {
            sub: user.id.to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::days(7))
                .naive_utc()
                .timestamp()
                .try_into()?,
            cid: active_credential.id,
        };
        let key_doc = self.get_active_key().await?;
        let token = jsonwebtoken::encode(
            &Header {
                kid: Some(key_doc.id.to_string()),
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_secret(key_doc.key.as_bytes()),
        )?;
        Ok(Session {
            token,
            user: user.to_owned(),
            credential_id: active_credential.id,
        })
    }

    pub async fn parse_from_token(&self, token: &str) -> anyhow::Result<Session> {
        let meta = jsonwebtoken::decode_header(token)?;
        let kid = meta.kid.context("session key id empty")?;
        let key_doc = SessionKey::find_by_id(kid.parse()?)
            .one(&self.db)
            .await?
            .context(format!("session key {} not found", kid))?;
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(key_doc.key.as_bytes()),
            &Validation::default(),
        )?
        .claims;
        if chrono::Utc::now().naive_utc()
            < chrono::NaiveDateTime::from_timestamp(claims.exp.try_into()?, 0)
        {
            let active_credential = self
                .credential
                .get_active_credential(claims.sub.parse()?)
                .await?;
            if claims.cid == active_credential.id {
                Ok(Session {
                    token: token.to_owned(),
                    user: User::find_by_id(claims.sub.parse()?)
                        .one(&self.db)
                        .await?
                        .context("user not found")?,
                    credential_id: claims.cid,
                })
            } else {
                anyhow::bail!("password has been changed since last login. please re-login");
            }
        } else {
            anyhow::bail!("session expired");
        }
    }
}
