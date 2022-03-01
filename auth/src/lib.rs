use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use session::SessionModule;
use std::error::Error;

mod session;

#[derive(Clone)]
pub struct AuthModule {
    session: SessionModule,
    db: DatabaseConnection,
}

// constructor
impl AuthModule {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            db: db_connection.clone(),
            session: SessionModule::new(db_connection.clone()),
        }
    }
}

// public api
impl AuthModule {
    pub async fn login(
        &self,
        name_or_email: &str,
        password: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let user = model::user::Entity::find()
            .filter(
                Condition::any()
                    .add(model::user::Column::Email.eq(name_or_email.to_owned()))
                    .add(model::user::Column::Name.eq(name_or_email.to_owned())),
            )
            .one(&self.db)
            .await?
            .ok_or("user not found")?;
        if user.check_password(password)? {
            Ok(self.session.serialize(user.id).await?)
        } else {
            Err("password incorrect".into())
        }
    }

    pub async fn sign_up(
        &self,
        name: &str,
        password: &str,
        email: Option<String>,
        avatar: Option<String>,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::user::ActiveModel {
            name: Set(name.to_owned()),
            password: Set(password.to_owned()),
            email: Set(email),
            avatar: Set(avatar),
            ..Default::default()
        }
        .insert(&self.db)
        .await?)
    }

    pub async fn get_user_for_session(
        &self,
        session: &str,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        Ok(self.session.deserialize(session).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use db::new_database_connection;

    use super::*;

    #[tokio::test]
    async fn login() -> Result<(), Box<dyn Error + Send + Sync>> {
        let username = "asdf".to_owned();
        let password = "zxcv".to_owned();
        let email = "email@test.com".to_owned();
        let module = get_module().await?;
        // fail for non-existing user
        assert_eq!(0, model::user::Entity::find().all(&module.db).await?.len());
        assert!(module.login(&username, &password).await.is_err());
        // succeed for existing user
        model::user::ActiveModel {
            name: Set(username.clone()),
            password: Set(password.clone()),
            email: Set(Some(email.clone())),
            ..Default::default()
        }
        .insert(&module.db)
        .await?;
        assert_eq!(1, model::user::Entity::find().all(&module.db).await?.len());
        assert!(module.login(&username, &password).await.is_ok());
        assert!(module.login(&email, &password).await.is_ok());
        // fail for wrong password
        assert!(module.login(&username, "qwer").await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn signup() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        // first user
        let name = "asdf".to_owned();
        let password = "asdf".to_owned();
        let email = "asdf".to_owned();
        assert_eq!(0, model::user::Entity::find().all(&module.db).await?.len());
        assert!(module
            .sign_up(&name, &password, Some(email.clone()), None)
            .await
            .is_ok());
        assert_eq!(1, model::user::Entity::find().all(&module.db).await?.len());
        model::user::Entity::find()
            .filter(model::user::Column::Name.eq(name.clone()))
            .one(&module.db)
            .await?
            .ok_or("signed up user not found")?;
        // cannot signup with same user name
        assert!(module.sign_up(&name, &password, None, None).await.is_err());
        assert_eq!(1, model::user::Entity::find().all(&module.db).await?.len());
        // cannot signup with same email
        assert!(module
            .sign_up("zxcv", &password, Some(email.clone()), None)
            .await
            .is_err());
        assert_eq!(1, model::user::Entity::find().all(&module.db).await?.len());
        Ok(())
    }

    #[tokio::test]
    async fn deserialize_session() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        // can deserialize valid session
        Ok(())
    }

    async fn get_module() -> Result<AuthModule, Box<dyn Error + Send + Sync>> {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        let db = new_database_connection().await?;
        Ok(AuthModule::new(db))
    }
}
