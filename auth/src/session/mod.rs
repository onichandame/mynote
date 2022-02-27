use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use model;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Add};

#[derive(Clone)]
pub struct SessionModule {
    db: DatabaseConnection,
}

// constructor
impl SessionModule {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db: db_connection }
    }
}

// public api
impl SessionModule {
    pub async fn create_session_for_user(
        &self,
        uid: i32,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let claims = Claims {
            sub: uid.to_string(),
            exp: usize::try_from(Utc::now().add(Duration::days(31)).naive_utc().timestamp())?,
        };
        let key_doc = self.get_latest_key().await?;
        Ok(jsonwebtoken::encode(
            &Header {
                kid: Some(key_doc.id.to_string()),
                ..Default::default()
            },
            &claims,
            &EncodingKey::from_secret(key_doc.key.as_ref()),
        )?)
    }

    pub async fn deserialize_session(
        &self,
        session: &str,
    ) -> Result<model::user::Model, Box<dyn Error + Sync + Send>> {
        let meta = jsonwebtoken::decode_header(session)?;
        let kid = meta.kid.ok_or("session key id empty")?;
        let key_doc = model::session_key::Entity::find_by_id(kid.parse()?)
            .one(&self.db)
            .await?
            .ok_or("session key not found")?;
        let claim = jsonwebtoken::decode::<Claims>(
            session,
            &DecodingKey::from_secret(&key_doc.key.as_ref()),
            &Validation::default(),
        )?
        .claims;
        let uid = claim.sub;
        Ok(model::user::Entity::find_by_id(uid.parse()?)
            .one(&self.db)
            .await?
            .ok_or("user not found")?)
    }
}

// private methods
impl SessionModule {
    async fn get_latest_key(
        &self,
    ) -> Result<model::session_key::Model, Box<dyn Error + Sync + Send>> {
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
            .insert(&self.db)
            .await?)
        };
        match model::session_key::Entity::find()
            .order_by_desc(model::session_key::Column::CreatedAt)
            .one(&self.db)
            .await?
        {
            Some(key) => {
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
            None => create_key().await,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[cfg(test)]
mod tests {
    use std::env;

    use db::new_database_connection;

    use super::*;

    #[tokio::test]
    async fn get_latest_key() -> Result<(), Box<dyn Error + Send + Sync>> {
        let db = get_db().await?;
        Ok(())
    }

    async fn get_db() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        Ok(new_database_connection().await?)
    }
}
