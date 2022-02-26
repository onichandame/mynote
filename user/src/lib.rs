use std::error::Error;

use model::conversion::IntoActiveValue;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Unchanged};

#[derive(Clone)]
pub struct UserModule {
    db: DatabaseConnection,
}

/// constructor
impl UserModule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

/// public api
impl UserModule {
    pub async fn get(&self, id: i32) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(format!("user {} not found", id))?)
    }

    pub async fn update(
        &self,
        id: i32,
        name: Option<String>,
        password: Option<String>,
        email: Option<Option<String>>,
        avatar: Option<Option<String>>,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::user::ActiveModel {
            id: Unchanged(id),
            name: name.into_active_value(),
            password: password.into_active_value(),
            email: email.into_active_value(),
            avatar: avatar.into_active_value(),
            ..Default::default()
        }
        .update(&self.db)
        .await?)
    }
}
