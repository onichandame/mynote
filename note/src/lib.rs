use std::error::Error;

use model::conversion::IntoActiveValue;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};

mod filter;
pub use filter::*;

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
        offset: Option<u64>,
        filter: Option<Filter>,
    ) -> Result<Vec<model::note::Model>, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_pagination(query, offset, limit);
        query = self.apply_filter(query, filter);
        Ok(query.all(&self.db).await?)
    }
    pub async fn get(
        &self,
        filter: Filter,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, Some(filter));
        Ok(query
            .one(&self.db)
            .await?
            .ok_or(format!("note not found",))?)
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
        filter: Filter,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let update = model::note::ActiveModel {
            title: title.into_active_value(),
            content: content.into_active_value(),
            ..Default::default()
        };
        let mut query = model::note::Entity::update_many().set(update);
        query = self.apply_filter(query, Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
    pub async fn delete(&self, filter: Filter) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::delete_many();
        query = self.apply_filter(query, Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
}

// private methods
impl NoteModule {
    fn apply_pagination<T: QuerySelect>(
        &self,
        mut query: T,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> T {
        if let Some(offset) = offset {
            query = query.offset(offset)
        }
        if let Some(limit) = limit {
            query = query.limit(limit)
        }
        query
    }
    fn apply_filter<T: QueryFilter>(&self, mut query: T, filter: Option<Filter>) -> T {
        if let Some(filter) = filter {
            if let Some(user_id) = filter.user_id {
                query = query.filter(model::note::Column::UserId.eq(user_id));
            }
        }
        query
    }
}
