use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use std::error::Error;

use model;

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
    pub async fn serialize(
        &self,
        user: &model::user::Model,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let claims = Claims {
            sub: user.id.to_string(),
            exp: usize::try_from(
                (Utc::now() + self.get_session_ttl())
                    .naive_utc()
                    .timestamp(),
            )?,
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

    pub async fn deserialize(
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
                    .gt(&self.get_key_ttl())
                {
                    create_key().await
                } else {
                    Ok(key)
                }
            }
            None => create_key().await,
        }
    }

    fn get_key_ttl(&self) -> chrono::Duration {
        chrono::Duration::days(30)
    }

    fn get_session_ttl(&self) -> chrono::Duration {
        chrono::Duration::days(7)
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
    use sea_orm::{sea_query::Expr, ModelTrait};

    use super::*;

    #[tokio::test]
    async fn get_latest_key() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        let get_keys_count = || async {
            Ok::<usize, Box<dyn Error + Send + Sync>>(
                model::session_key::Entity::find()
                    .all(&module.db)
                    .await?
                    .len(),
            )
        };
        // first call
        assert_eq!(0, get_keys_count().await?);
        let first_key = module.get_latest_key().await?;
        assert_eq!(1, get_keys_count().await?);
        // second call
        let second_key = module.get_latest_key().await?;
        assert_eq!(1, get_keys_count().await?);
        assert_eq!(first_key.key, second_key.key);
        // first call after the latest has expired
        model::session_key::Entity::update_many()
            .col_expr(
                model::session_key::Column::CreatedAt,
                Expr::value(chrono::Utc::now().naive_utc() - module.get_key_ttl() * 2),
            )
            .exec(&module.db)
            .await?;
        let third_key = module.get_latest_key().await?;
        assert_eq!(2, get_keys_count().await?);
        assert_ne!(second_key.key, third_key.key);
        // second call after the latest has expired
        let forth_key = module.get_latest_key().await?;
        assert_eq!(2, get_keys_count().await?);
        assert_eq!(forth_key.key, third_key.key);
        // expired key can be retrieved by id
        assert_eq!(
            first_key.key,
            model::session_key::Entity::find_by_id(first_key.id)
                .one(&module.db)
                .await?
                .ok_or("failed to find expired key")?
                .key
        );
        Ok(())
    }

    #[tokio::test]
    async fn session() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        let user = model::user::ActiveModel {
            name: Set("".to_owned()),
            password: Set("".to_owned()),
            ..Default::default()
        }
        .insert(&module.db)
        .await?;
        // serialize for valid user
        let session = module.serialize(&user).await?;
        // desialize for valid user
        assert_eq!(user.id, module.deserialize(&session).await?.id);
        // desialize for invalid session
        assert!(module.deserialize("").await.is_err());
        // desialize for invalid user
        user.delete(&module.db).await?;
        assert!(module.deserialize(&session).await.is_err());
        Ok(())
    }

    async fn get_module() -> Result<SessionModule, Box<dyn Error + Send + Sync>> {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        Ok(SessionModule::new(new_database_connection().await?.clone()))
    }
}
