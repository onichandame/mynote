use std::error::Error;

use model::conversion::IntoActiveValue;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, QuerySelect, Set, Unchanged,
};

#[derive(Clone)]
pub struct NoteModule {
    db: DatabaseConnection,
}

/// constructor
impl NoteModule {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self {
            db: db_connection.clone(),
        }
    }
}

/// public api
impl NoteModule {
    /// TODO: filter/sort/pagination
    pub async fn list(
        &self,
        limit: Option<u64>,
    ) -> Result<Vec<model::note::Model>, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        match limit {
            None => {}
            Some(limit) => {
                query = query.limit(limit);
            }
        }
        Ok(query.all(&self.db).await?)
    }
    pub async fn get(&self, id: i32) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::note::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(format!("note {} not found", id))?)
    }
    pub async fn create(
        &self,
        user: i32,
        title: &str,
        content: &str,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::note::ActiveModel {
            user_id: Set(user),
            title: Set(title.to_owned()),
            content: Set(content.to_owned()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?)
    }
    pub async fn update(
        &self,
        id: i32,
        user: Option<i32>,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::note::ActiveModel {
            id: Unchanged(id),
            user_id: user.into_active_value(),
            title: title.into_active_value(),
            content: content.into_active_value(),
            ..Default::default()
        }
        .update(&self.db)
        .await?)
    }
    pub async fn delete(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        let doc = self.get(id).await?;
        Ok(doc.delete(&self.db).await.map(|_| ())?)
    }
}
