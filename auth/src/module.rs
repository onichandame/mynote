use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::error::Error;

pub struct AuthModule<'a> {
    db: &'a DatabaseConnection,
}

// constructor
impl<'a> AuthModule<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

// public api
impl AuthModule<'_> {
    pub async fn login_by_password(
        &self,
        identity: &str,
        password: &str,
    ) -> Result<model::user::Model, Box<dyn Error + Send + Sync>> {
        let user = model::user::Entity::find()
            .filter(
                Condition::any()
                    .add(model::user::Column::Email.eq(identity.to_owned()))
                    .add(model::user::Column::Name.eq(identity.to_owned())),
            )
            .one(self.db)
            .await?
            .ok_or("user not found")?;
        if user.check_password(password)? {
            Ok(user)
        } else {
            Err("password incorrect".into())
        }
    }

    pub async fn signup(
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
        .insert(self.db)
        .await?)
    }
}

#[cfg(test)]
mod tests {
    use config::{ConfigModule, Mode};
    use db::DbModule;

    use super::*;

    #[tokio::test]
    async fn login() -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = ConfigModule::create(Mode::UnitTest)?;
        let db = DbModule::create(&config).await?;
        let auth = AuthModule::new(&db);

        let username = "asdf".to_owned();
        let password = "zxcv".to_owned();
        let email = "email@test.com".to_owned();
        // fail for non-existing user
        assert_eq!(0, model::user::Entity::find().all(&db).await?.len());
        assert!(auth.login_by_password(&username, &password).await.is_err());
        // succeed for existing user
        model::user::ActiveModel {
            name: Set(username.clone()),
            password: Set(password.clone()),
            email: Set(Some(email.clone())),
            ..Default::default()
        }
        .insert(&db)
        .await?;
        assert_eq!(1, model::user::Entity::find().all(&db).await?.len());
        assert!(auth.login_by_password(&username, &password).await.is_ok());
        assert!(auth.login_by_password(&email, &password).await.is_ok());
        // fail for wrong password
        assert!(auth.login_by_password(&username, "qwer").await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn signup() -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = ConfigModule::create(Mode::UnitTest)?;
        let db = DbModule::create(&config).await?;
        let auth = AuthModule::new(&db);
        // first user
        let name = "asdf".to_owned();
        let password = "asdf".to_owned();
        let email = "asdf".to_owned();
        assert_eq!(0, model::user::Entity::find().all(&db).await?.len());
        assert!(auth
            .signup(&name, &password, Some(email.clone()), None)
            .await
            .is_ok());
        assert_eq!(1, model::user::Entity::find().all(&db).await?.len());
        model::user::Entity::find()
            .filter(model::user::Column::Name.eq(name.clone()))
            .one(auth.db)
            .await?
            .ok_or("signed up user not found")?;
        // cannot signup with same user name
        assert!(auth.signup(&name, &password, None, None).await.is_err());
        assert_eq!(1, model::user::Entity::find().all(&db).await?.len());
        // cannot signup with same email
        assert!(auth
            .signup("zxcv", &password, Some(email.clone()), None)
            .await
            .is_err());
        assert_eq!(1, model::user::Entity::find().all(&db).await?.len());
        Ok(())
    }
}
