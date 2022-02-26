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
            Ok(self.session.create_session_for_user(user.id).await?)
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
        Ok(self.session.deserialize_session(session).await?)
    }
}
