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

#[cfg(test)]
mod tests {
    use std::env;

    use db::new_database_connection;
    use sea_orm::Set;

    use super::*;
    #[tokio::test]
    async fn user() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        // get user by id
        let user = model::user::ActiveModel {
            name: Set("".to_owned()),
            password: Set("".to_owned()),
            ..Default::default()
        }
        .insert(&module.db)
        .await?;
        assert_eq!(user.id, module.get(user.id).await?.id);
        // update user
        let name = "asdf".to_owned();
        let updated_user = module
            .update(user.id, Some(name.clone()), None, None, None)
            .await?;
        assert_eq!(name, updated_user.name);
        assert_eq!(user.id, updated_user.id);
        assert_eq!(updated_user.name, module.get(updated_user.id).await?.name);
        Ok(())
    }

    async fn get_module() -> Result<UserModule, Box<dyn Error + Send + Sync>> {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        let db = new_database_connection().await?;
        Ok(UserModule::new(db))
    }
}
