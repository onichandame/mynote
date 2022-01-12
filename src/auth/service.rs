use async_graphql;
use bcrypt;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::{self, distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx;
use std::{error::Error, ops::Add, str};

use crate::db::{self, model};

use super::{error, Session};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    name_or_email: &String,
    password: &String,
    pool: &db::ConnectionPool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let user: model::User =
        sqlx::query_as::<_, model::User>("SELECT * FROM users WHERE email = ? OR name = ?")
            .bind(name_or_email)
            .bind(name_or_email)
            .fetch_one(pool)
            .await?;
    let valid = bcrypt::verify(password, &user.password)?;
    if valid {
        Ok(create_session(user.id, pool).await)
    } else {
        Err(Box::new(error::AuthError::IncorrectPassword))
    }
}

pub async fn create_session(uid: i64, pool: &db::ConnectionPool) -> String {
    let claims = Claims {
        sub: uid.to_string(),
        exp: usize::try_from(Utc::now().add(Duration::days(31)).naive_utc().timestamp()).unwrap(),
    };
    let key = get_latest_key(pool).await.unwrap();
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
) -> async_graphql::Result<model::User> {
    let pool = ctx.data::<db::ConnectionPool>().unwrap();
    let session = ctx
        .data::<Session>()
        .map_err(|_| async_graphql::Error::new("Unauthorized"))?;
    Ok(deserialize_session(session, pool).await?)
}

pub async fn deserialize_session(
    session: &str,
    pool: &db::ConnectionPool,
) -> Result<model::User, Box<dyn Error + Sync + Send>> {
    let meta = jsonwebtoken::decode_header(session)?;
    let kid = meta.kid.ok_or(sqlx::Error::RowNotFound)?;
    let key = sqlx::query_as::<_, model::SessionKey>("SELECT * FROM session_keys WHERE id = ?")
        .bind(kid)
        .fetch_one(pool)
        .await?;
    let claim = jsonwebtoken::decode::<Claims>(
        session,
        &DecodingKey::from_secret(&key.key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| sqlx::Error::RowNotFound)?
    .claims;
    let uid = claim.sub;
    let user = sqlx::query_as::<_, model::User>("SELECT * FROM users WHERE id = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

async fn get_latest_key(
    pool: &db::ConnectionPool,
) -> Result<model::SessionKey, Box<dyn Error + Sync + Send>> {
    let key_or_err = sqlx::query_as::<_, model::SessionKey>(
        "SELECT * from session_keys ORDER BY created_at DESC",
    )
    .fetch_one(pool)
    .await;
    let create_key = || async {
        let key = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();
        Ok(sqlx::query_as::<_, model::SessionKey>(
            "INSERT INTO session_keys (key) VALUES (?) RETURNING *",
        )
        .bind(&key)
        .fetch_one(pool)
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
            sqlx::error::Error::RowNotFound => create_key().await,
            other => Err(other.into()),
        },
    }
}
