use std::error::Error;

use sea_orm::{ActiveModelTrait, DatabaseConnection, Unchanged};

#[derive(Clone)]
pub struct UserModule {
    db: DatabaseConnection,
}

// constructor
impl UserModule {
    pub fn new(db_connection: DatabaseConnection) -> UserModule {
        UserModule {
            db: db_connection.clone(),
        }
    }
}

// public api
impl UserModule {
    pub async fn create_user(
        &self,
        input: model::user::ActiveModel,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        Ok(input.insert(&self.db).await?)
    }

    pub async fn update_user(
        &self,
        id: i32,
        mut update: model::user::ActiveModel,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        update.id = Unchanged(id);
        Ok(update.update(&self.db).await?)
    }
}
