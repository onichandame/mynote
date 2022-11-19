use anyhow::Context;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use crate::entity::{credential, prelude::*};

#[derive(Clone)]
pub struct CredentialModule {
    db: DatabaseConnection,
}

impl CredentialModule {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn update(&self, user_id: i32, password: &str) -> anyhow::Result<()> {
        credential::ActiveModel {
            user_id: Set(user_id),
            password: Set(bcrypt::hash(password, bcrypt::DEFAULT_COST)?),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;
        Ok(())
    }

    pub async fn check(&self, user_id: i32, password: &str) -> anyhow::Result<bool> {
        let cred = self.get_active_credential(user_id).await?;
        Ok(bcrypt::verify(password, &cred.password)?)
    }

    pub async fn try_get_active_credential(
        &self,
        user_id: i32,
    ) -> anyhow::Result<Option<credential::Model>> {
        Ok(Credential::find()
            .filter(credential::Column::UserId.eq(user_id))
            .order_by_desc(credential::Column::Id)
            .one(&self.db)
            .await?)
    }

    pub async fn get_active_credential(&self, user_id: i32) -> anyhow::Result<credential::Model> {
        Ok(self
            .try_get_active_credential(user_id)
            .await?
            .context("user does not have password set. please contact the admin")?)
    }
}
