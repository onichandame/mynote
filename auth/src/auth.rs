use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use std::error::Error;

use crate::session::SessionModule;

#[derive(Clone)]
pub struct AuthModule {
    session_module: SessionModule,
    db: DatabaseConnection,
}

// constructor
impl AuthModule {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            db: db_connection.clone(),
            session_module: SessionModule::new(db_connection.clone()),
        }
    }
}

// public api
impl AuthModule {
    pub async fn login(
        &self,
        name_or_email: &String,
        password: &String,
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
            Ok(self.session_module.create_session_for_user(user.id).await?)
        } else {
            Err("password incorrect".into())
        }
    }

    pub async fn get_user_from_ctx(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> async_graphql::Result<model::user::Model> {
        let session = ctx
            .data::<Session>()
            .map_err(|_| async_graphql::Error::new("Unauthorized"))?;
        Ok(self.session_module.deserialize_session(session).await?)
    }
}

pub type Session = String;
