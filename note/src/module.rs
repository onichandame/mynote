use std::error::Error;

use model::conversion::IntoActiveValue;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};


#[derive(Clone)]
pub struct NoteModule<'a> {
    db: &'a DatabaseConnection,
}

/// constructor
impl <'a>NoteModule <'a>{
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self {
            db,
        }
    }
}

/// public api
impl NoteModule <'_>{
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
        Ok(query.all(self.db).await?)
    }
    pub async fn get(
        &self,
        filter: Filter,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, Some(filter));
        Ok(query
            .one(self.db)
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
        .insert(self.db)
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
        Ok(query.exec(self.db).await.map(|_| ())?)
    }
    pub async fn delete(&self, filter: Filter) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::delete_many();
        query = self.apply_filter(query, Some(filter));
        Ok(query.exec(self.db).await.map(|_| ())?)
    }
}

// private methods
impl NoteModule <'_>{
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
            if let Some(id) = filter.id {
                query = query.filter(model::note::Column::Id.eq(id));
            }
            if let Some(deleted_at_maybe) = filter.deleted_at {
                query = match deleted_at_maybe {
                    Some(deleted_at_filter) => {
                        if let Some(gt) = deleted_at_filter.gt {
                            query = query.filter(model::note::Column::DeletedAt.gt(gt));
                        }
                        if let Some(gte) = deleted_at_filter.gte {
                            query = query.filter(model::note::Column::DeletedAt.gte(gte));
                        }
                        if let Some(lt) = deleted_at_filter.lt {
                            query = query.filter(model::note::Column::DeletedAt.gt(lt));
                        }
                        if let Some(lte) = deleted_at_filter.lte {
                            query = query.filter(model::note::Column::DeletedAt.gte(lte));
                        }
                        query
                    }
                    None => query.filter(model::note::Column::DeletedAt.is_null()),
                };
            }
        }
        query
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[tokio::test]
    async fn note() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        let count_notes = || async {
            Ok::<usize, Box<dyn Error + Send + Sync>>(
                model::note::Entity::find().all(&module.db).await?.len(),
            )
        };
        // create
        let user = model::user::ActiveModel {
            name: Set("".to_owned()),
            password: Set("".to_owned()),
            ..Default::default()
        }
        .insert(&module.db)
        .await?;
        assert_eq!(0, count_notes().await?);
        let created_note = module.create(user.id, "", "").await?;
        assert_eq!(1, count_notes().await?);
        // get
        let got_note = module
            .get(Filter {
                id: Some(created_note.id),
                ..Default::default()
            })
            .await?;
        assert_eq!(created_note.id, got_note.id);
        // list
        let listed_notes = module.list(None, None, None).await?;
        assert_eq!(1, listed_notes.len());
        assert_eq!(
            created_note.id,
            listed_notes.get(0).ok_or("fatal error")?.id
        );
        // limit
        assert_eq!(0, module.list(Some(0), None, None).await?.len());
        // offset
        assert_eq!(0, module.list(Some(1), Some(1), None).await?.len());
        // update
        let title = "asdf".to_owned();
        assert!(module
            .update(
                Filter {
                    id: Some(created_note.id),
                    ..Default::default()
                },
                Some(title.clone()),
                None,
            )
            .await
            .is_ok());
        assert_eq!(
            title,
            model::note::Entity::find_by_id(created_note.id)
                .one(&module.db)
                .await?
                .ok_or("fatal error")?
                .title
        );
        // delete
        assert!(module
            .delete(Filter {
                id: Some(created_note.id),
                ..Default::default()
            })
            .await
            .is_ok());
        assert!(model::note::Entity::find_by_id(created_note.id)
            .one(&module.db)
            .await?
            .is_none());
        Ok(())
    }

    async fn filter() -> Result<(), Box<dyn Error + Send + Sync>> {
        let module = get_module().await?;
        let user = model::user::ActiveModel {
            name: Set(""),
            password: Set(""),
            ..Default::default()
        }
        .insert(&module.db)
        .await?;
        let note = module.create(user.id, "", "").await?;
        Ok(())
    }

    async fn get_module() -> Result<NoteModule, Box<dyn Error + Send + Sync>> {
        env::set_var("DATABASE_URL", "sqlite://:memory:");
        let db = new_database_connection().await?;
        Ok(NoteModule::new(db))
    }
}
