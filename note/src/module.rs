use std::error::Error;

use chrono::NaiveDateTime;
use model::conversion::IntoActiveValue;
use pagination::Pagination;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use sorting::Sorting;

use crate::NoteFilter;

#[derive(Clone)]
pub struct NoteModule {
    db: DatabaseConnection,
}

/// constructor
pub fn new_note_module(db: DatabaseConnection) -> NoteModule {
    NoteModule { db }
}

/// public api
impl NoteModule {
    pub async fn list(
        &self,
        filter: Option<NoteFilter>,
        pagination: Option<Pagination>,
        sorting: Option<Vec<Sorting>>,
    ) -> Result<Vec<model::note::Model>, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_pagination(query, &pagination);
        query = self.apply_filter(query, &filter);
        query = self.apply_sorting(query, &sorting);
        Ok(query.all(&self.db).await?)
    }
    pub async fn count(
        &self,
        filter: Option<NoteFilter>,
    ) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, &filter);
        Ok(query.count(&self.db).await?)
    }
    pub async fn get(
        &self,
        filter: NoteFilter,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, &Some(filter));
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
        uuid: Option<String>,
        lamport_clock: Option<i32>,
    ) -> Result<model::note::Model, Box<dyn Error + Send + Sync>> {
        Ok(model::note::ActiveModel {
            user_id: Set(user),
            title: Set(title.to_owned()),
            content: Set(content.to_owned()),
            uuid: match uuid {
                Some(v) => Set(v),
                None => NotSet,
            },
            lamport_clock: match lamport_clock {
                Some(v) => Set(v),
                None => NotSet,
            },
            ..Default::default()
        }
        .insert(&self.db)
        .await?)
    }
    pub async fn update(
        &self,
        filter: NoteFilter,
        title: Option<String>,
        content: Option<String>,
        deleted_at: Option<Option<NaiveDateTime>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let update = model::note::ActiveModel {
            title: title.into_active_value(),
            content: content.into_active_value(),
            deleted_at: deleted_at.into_active_value(),
            ..Default::default()
        };
        let mut query = model::note::Entity::update_many().set(update);
        query = self.apply_filter(query, &Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
    pub async fn delete(&self, filter: NoteFilter) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut query = model::note::Entity::delete_many();
        query = self.apply_filter(query, &Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
}

// private methods
impl NoteModule {
    fn apply_pagination<T: QuerySelect>(&self, mut query: T, pagination: &Option<Pagination>) -> T {
        if let Some(pagination) = pagination {
            query = pagination.build(query)
        }
        query
    }
    fn apply_filter<T: QueryFilter>(&self, mut query: T, filter: &Option<NoteFilter>) -> T {
        if let Some(filter) = filter {
            query = filter.apply_filter(query);
        }
        query
    }
    fn apply_sorting<T: QueryOrder>(&self, mut query: T, sorting: &Option<Vec<Sorting>>) -> T {
        if let Some(sortings) = sorting {
            for sorting in sortings {
                match sorting.field.as_str() {
                    "created_at" => {
                        query = sorting.build(query, model::note::Column::CreatedAt);
                    }
                    _ => {}
                };
            }
        }
        query
    }
}

#[cfg(test)]
mod tests {
    use config::{new_config_provider, Mode};
    use db::new_db_connection;
    use filter::Filter;

    use super::*;

    #[tokio::test]
    async fn note() -> Result<(), Box<dyn Error + Send + Sync>> {
        let note = init().await?;
        let count_notes = || async {
            Ok::<usize, Box<dyn Error + Send + Sync>>(
                model::note::Entity::find().count(&note.db).await?,
            )
        };
        // create
        let user = model::user::ActiveModel {
            name: Set("".to_owned()),
            password: Set("".to_owned()),
            ..Default::default()
        }
        .insert(&note.db)
        .await?;
        assert_eq!(0, count_notes().await?);
        let created_note = note.create(user.id, "", "", None, None).await?;
        assert_eq!(1, count_notes().await?);
        // get
        let got_note = note
            .get(NoteFilter {
                id: Some(Filter {
                    eq: Some(created_note.id),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await?;
        assert_eq!(created_note.id, got_note.id);
        // list
        let listed_notes = note.list(None, None, None).await?;
        assert_eq!(1, listed_notes.len());
        assert_eq!(
            created_note.id,
            listed_notes.get(0).ok_or("fatal error")?.id
        );
        // count
        assert_eq!(1, note.count(None).await?);
        // filter
        assert_eq!(
            0,
            note.count(Some(NoteFilter {
                id: Some(Filter {
                    lt: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            }),)
                .await?
        );
        // pagination
        assert_eq!(
            0,
            note.list(
                None,
                Some(Pagination {
                    limit: Some(0),
                    ..Default::default()
                }),
                None
            )
            .await?
            .len()
        );
        assert_eq!(
            0,
            note.list(
                None,
                Some(Pagination {
                    limit: Some(1),
                    offset: Some(1),
                    ..Default::default()
                }),
                None
            )
            .await?
            .len()
        );
        // sorting
        let second_note = note.create(user.id, "", "", None, None).await?;
        assert_eq!(
            created_note.id,
            note.list(
                None,
                None,
                Some(vec!(Sorting {
                    field: "created_at".to_owned(),
                    direction: sorting::SortDirection::ASC
                }))
            )
            .await?
            .get(0)
            .ok_or("fatal error")?
            .id
        );
        assert_eq!(
            second_note.id,
            note.list(
                None,
                None,
                Some(vec!(Sorting {
                    field: "created_at".to_owned(),
                    direction: sorting::SortDirection::DESC
                }))
            )
            .await?
            .get(0)
            .ok_or("fatal error")?
            .id
        );
        // update
        let title = "asdf".to_owned();
        assert!(note
            .update(
                NoteFilter {
                    id: Some(Filter {
                        eq: Some(created_note.id),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Some(title.clone()),
                None,
                None
            )
            .await
            .is_ok());
        assert_eq!(
            title,
            model::note::Entity::find_by_id(created_note.id)
                .one(&note.db)
                .await?
                .ok_or("fatal error")?
                .title
        );
        // delete
        assert!(note
            .delete(NoteFilter {
                id: Some(Filter {
                    eq: Some(created_note.id),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await
            .is_ok());
        assert!(model::note::Entity::find_by_id(created_note.id)
            .one(&note.db)
            .await?
            .is_none());
        Ok(())
    }

    async fn init() -> Result<NoteModule, Box<dyn Error + Send + Sync>> {
        let config = new_config_provider(Mode::UnitTest)?;
        let db = new_db_connection(config.clone()).await?;
        Ok(new_note_module(db.clone()))
    }
}
