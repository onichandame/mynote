use std::error::Error;

use model::conversion::IntoActiveValue;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Unchanged};

#[derive(Clone)]
pub struct UserModule {
    db: DatabaseConnection,
}

/// constructor
pub fn new_user_module(db: DatabaseConnection) -> UserModule {
    UserModule { db }
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
    use config::{new_config_provider, Mode};
    use db::new_db_connection;
    use sea_orm::Set;

    use super::*;
    #[tokio::test]
    async fn user() -> Result<(), Box<dyn Error + Send + Sync>> {
        let user = init().await?;
        // get user by id
        let mocked_user = model::user::ActiveModel {
            name: Set("".to_owned()),
            password: Set("".to_owned()),
            ..Default::default()
        }
        .insert(&user.db)
        .await?;
        assert_eq!(mocked_user.id, user.get(mocked_user.id).await?.id);
        // update user
        let name = "asdf".to_owned();
        let updated_user = user
            .update(mocked_user.id, Some(name.clone()), None, None, None)
            .await?;
        assert_eq!(name, updated_user.name);
        assert_eq!(mocked_user.id, updated_user.id);
        assert_eq!(updated_user.name, user.get(updated_user.id).await?.name);
        Ok(())
    }

    async fn init() -> Result<UserModule, Box<dyn Error + Send + Sync>> {
        let config = new_config_provider(Mode::UnitTest)?;
        let db = new_db_connection(config.clone()).await?;
        Ok(new_user_module(db.clone()))
    }
}
