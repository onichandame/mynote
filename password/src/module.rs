use model::conversion::IntoActiveValue;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{result::Result, PasswordGroupFilter};

pub struct PasswordModule {
    db: DatabaseConnection,
}

/// constructor
pub fn new_password_module(db: DatabaseConnection) -> PasswordModule {
    PasswordModule { db }
}

/// public api
impl PasswordModule {
    pub async fn create_group(
        &self,
        user: i32,
        parent: Option<i32>,
        title: &str,
    ) -> Result<model::password_group::Model> {
        Ok(model::password_group::ActiveModel {
            user_id: user.into_active_value(),
            parent_id: parent.into_active_value(),
            title: title.to_owned().into_active_value(),
            ..Default::default()
        }
        .insert(&self.db)
        .await?)
    }
    pub async fn update_group(
        &self,
        filter: PasswordGroupFilter,
        title: Option<String>,
        parent: Option<Option<i32>>,
    ) -> Result<()> {
        let update = model::password_group::ActiveModel {
            title: title.into_active_value(),
            parent_id: parent.into_active_value(),
            ..Default::default()
        };
        let mut query = model::password_group::Entity::update_many().set(update);
        query = self.apply_filter(query, &Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
    pub async fn list_groups(
        &self,
        filter: Option<PasswordGroupFilter>,
    ) -> Result<Vec<model::password_group::Model>> {
        let mut query = model::password_group::Entity::find();
        query = self.apply_filter(query, &filter);
        Ok(query.all(&self.db).await?)
    }
}

/// private apis
impl PasswordModule {
    fn apply_filter<T: QueryFilter + Clone>(
        &self,
        query: T,
        filter: &Option<PasswordGroupFilter>,
    ) -> T {
        filter
            .as_ref()
            .map_or(query.clone(), |v| query.filter(v.build()))
    }
}
