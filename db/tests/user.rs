use db::{self, model};
use sqlx;
use std::error::Error;
use tokio;

mod setup;

#[tokio::test]
async fn create() -> Result<(), Box<dyn Error>> {
    let pool = setup::get_connection_pool().await;
    let name = "name";
    let password = "password";
    let email = "email";
    let user = sqlx::query_as::<_, model::User>(
        "INSERT INTO users (name, email, password) VALUES (?,?,?) RETURNING *",
    )
    .bind(name)
    .bind(email)
    .bind(password)
    .fetch_one(&pool)
    .await?;
    assert_eq!(user.name, name);
    assert_eq!(user.password, password);
    assert_eq!(user.email, Some(String::from(email)));
    Ok(())
}

#[tokio::test]
async fn list() -> Result<(), Box<dyn Error>> {
    let pool = setup::get_connection_pool().await;
    sqlx::query("INSERT INTO users (name, email, password) VALUES (?,?,?) RETURNING *")
        .bind("1")
        .bind("1")
        .bind("1")
        .execute(&pool)
        .await?;
    sqlx::query("INSERT INTO users (name, email, password) VALUES (?,?,?) RETURNING *")
        .bind("2")
        .bind("2")
        .bind("2")
        .execute(&pool)
        .await?;
    let users = sqlx::query_as::<_, model::User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;
    assert_eq!(users.len(), 2);
    Ok(())
}
