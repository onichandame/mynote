use async_graphql::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn signup(
    name: &str,
    password: &str,
    email: Option<&str>,
    db: &DatabaseConnection,
) -> Result<entity::user::Model> {
    let txn = db.begin().await?;
    let active_model = entity::user::ActiveModel {
        name: Set(name.to_owned()),
        email: email.map_or(sea_orm::ActiveValue::NotSet, |email| {
            Set(Some(email.to_owned()))
        }),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    let user = active_model.insert(&txn).await?;
    entity::credential::ActiveModel {
        user_id: Set(user.id),
        password: Set(password.to_owned()),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;
    txn.commit().await?;
    Ok(user)
}
