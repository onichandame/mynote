use std::error::Error;

use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

#[derive(Clone)]
pub struct PassModule {
    db: DatabaseConnection,
}

/// constructor
pub fn new_pass_module(db: DatabaseConnection) -> PassModule {
    PassModule { db }
}

/// public api
impl PassModule {
    pub async fn create(
        &self,
        user: i32,
        name: &str,
        password: &str,
    ) -> Result<model::password::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::password::ActiveModel {
            user_id: Set(user),
            name: Set(name.to_owned()),
            password: Set(password.to_owned()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?)
    }
}
