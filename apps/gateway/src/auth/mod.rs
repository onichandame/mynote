use anyhow::Context;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set,
};

use crate::{
    config::Config,
    entity::{self, prelude::*},
};

use self::{
    credential::CredentialModule,
    session::{Session, SessionModule},
};

pub mod credential;
pub mod session;

#[derive(Clone)]
pub struct AuthModule {
    pub credential: CredentialModule,
    pub session: SessionModule,
    db: DatabaseConnection,
    config: Config,
}

impl AuthModule {
    pub fn new(db: DatabaseConnection, config: Config) -> Self {
        let credential = CredentialModule::new(db.clone());
        let session = SessionModule::new(credential.clone(), db.clone());
        Self {
            credential,
            session,
            config,
            db,
        }
    }

    pub async fn signup(
        &self,
        name: &str,
        password: &str,
        email: Option<&str>,
        invitation_key: Option<&str>,
    ) -> anyhow::Result<entity::user::Model> {
        if self.config.invitation_only {
            if let Some(_admin) = User::find().one(&self.db).await? {
                anyhow::bail!("Invitation-only restriction is turned on. Please contact the admin");
            }
        }
        let mut user_create = entity::user::ActiveModel {
            name: Set(name.to_owned()),
            email: email.map_or(sea_orm::ActiveValue::NotSet, |email| {
                Set(Some(email.to_owned()))
            }),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        if let Some(invitation_key) = invitation_key {
            let invitation = Invitation::find()
                .filter(entity::invitation::Column::Key.eq(invitation_key))
                .one(&self.db)
                .await?
                .context("Invitation key invalid")?;
            user_create.invitation_id = Set(Some(invitation.id));
        }
        let user = user_create.insert(&self.db).await?;
        self.credential.update(user.id, password).await?;
        Ok(user)
    }

    pub async fn login_by_password(
        &self,
        identity: &str,
        password: &str,
    ) -> anyhow::Result<Session> {
        let user = User::find()
            .filter(
                Condition::any()
                    .add(entity::user::Column::Email.eq(identity.clone()))
                    .add(entity::user::Column::Name.eq(identity.clone())),
            )
            .one(&self.db)
            .await?
            .context("user not found")?;
        if !self.credential.check(user.id, password).await? {
            anyhow::bail!("password incorrect");
        }
        Ok(self.session.generate_for_user(&user).await?)
    }
}
