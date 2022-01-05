use bcrypt;
use db::model;
use jwt_simple::prelude::{Claims, Duration, HS256Key, MACLike, NoCustomClaims, Token};
use sqlx;
use std::{error::Error, str};

use crate::error;

pub async fn create_user(
    name: &String,
    password: &String,
    email: &Option<String>,
    pool: &db::ConnectionPool,
) -> Result<model::User, Box<dyn Error + Send + Sync>> {
    Ok(sqlx::query_as::<_, model::User>(
        "INSERT INTO users (name,email,password) VALUES (?,?,?) RETURNING *",
    )
    .bind(name)
    .bind(email)
    .bind(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
    .fetch_one(pool)
    .await?)
}

pub async fn login(
    name_or_email: &String,
    password: &String,
    pool: &db::ConnectionPool,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let user: model::User = sqlx::query_as::<_, model::User>(
        "SELECT id,password FROM users WHERE email = ? OR name = ?",
    )
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
    let claims = Claims::create(Duration::from_days(31)).with_subject(uid);
    let key = get_latest_key(pool).await.unwrap();
    HS256Key::from_bytes(key.key.as_bytes())
        .with_key_id(&key.id.to_string())
        .authenticate(claims)
        .unwrap()
}

pub async fn deserialize_session(
    session: &str,
    pool: &db::ConnectionPool,
) -> Result<model::User, Box<dyn Error + Sync + Send>> {
    let meta = Token::decode_metadata(session).unwrap();
    let kid = meta.key_id().ok_or(sqlx::Error::RowNotFound)?;
    let key = sqlx::query_as::<_, model::SessionKey>("SELECT key FROM session_keys WHERE id = ?")
        .bind(kid)
        .fetch_one(pool)
        .await?;
    let claim =
        HS256Key::from_bytes(key.key.as_bytes()).verify_token::<NoCustomClaims>(session, None)?;
    let uid = claim.subject.ok_or(sqlx::Error::RowNotFound)?;
    Ok(
        sqlx::query_as::<_, model::User>("SELECT * FROM users WHERE id = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?,
    )
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
        Ok(sqlx::query_as::<_, model::SessionKey>(
            "INSERT INTO session_keys (key) VALUES (?) RETURNING *",
        )
        .bind(str::from_utf8(&HS256Key::generate().to_bytes())?)
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
