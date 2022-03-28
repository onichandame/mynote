use std::error::Error as StdError;

use async_trait::async_trait;
use chrono::NaiveDateTime;
use futures::{Stream, StreamExt};
use model::conversion::IntoActiveValue;
use pagination::Pagination;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, NotSet, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use sorting::Sorting;
use tokio_graphql_ws::{Client, ClientActor};

use crate::NoteFilter;

type Error = Box<dyn StdError + Send + Sync>;

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
    ) -> Result<Vec<model::note::Model>, Error> {
        let mut query = model::note::Entity::find();
        query = self.apply_pagination(query, &pagination);
        query = self.apply_filter(query, &filter);
        query = self.apply_sorting(query, &sorting);
        Ok(query.all(&self.db).await?)
    }
    pub async fn count(&self, filter: Option<NoteFilter>) -> Result<usize, Error> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, &filter);
        Ok(query.count(&self.db).await?)
    }
    pub async fn get(&self, filter: NoteFilter) -> Result<model::note::Model, Error> {
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
    ) -> Result<model::note::Model, Error> {
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
    ) -> Result<(), Error> {
        let update = model::note::ActiveModel {
            title: title.into_active_value(),
            content: content.into_active_value(),
            deleted_at: deleted_at.into_active_value(),
            updated_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };
        let mut query = model::note::Entity::update_many().set(update);
        query = self.apply_filter(query, &Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
    pub async fn delete(&self, filter: NoteFilter) -> Result<(), Error> {
        let mut query = model::note::Entity::delete_many();
        query = self.apply_filter(query, &Some(filter));
        Ok(query.exec(&self.db).await.map(|_| ())?)
    }
    pub async fn stream(
        &self,
        filter: NoteFilter,
    ) -> Result<impl Stream<Item = model::note::Model> + '_, Error> {
        let mut query = model::note::Entity::find();
        query = self.apply_filter(query, &Some(filter));
        Ok(query
            .stream(&self.db)
            .await?
            .filter_map(|v| async move { v.ok() }))
    }
    pub async fn sync_from(
        &self,
        url: &str,
        remote_username: &str,
        remote_password: &str,
        user_id: i32,
    ) -> Result<(), Error> {
        #[derive(Clone, Serialize)]
        struct Session {
            session: Option<String>,
        }
        #[async_trait]
        impl ClientActor for Session {
            async fn connection_init(&self) -> Result<Option<serde_json::Value>, Error> {
                Ok(Some(serde_json::to_value(self)?))
            }
        }
        let mut client = Client::new()
            .set_url(url)
            .set_actors(Session { session: None });
        let (mut fut, mut subscriber) = client.try_connect().await?;
        let mut connection = tokio::spawn(fut);
        let login_response = subscriber
            .subscribe(
                &format!(
                    "mutation login{{
                        login(input:{{identity:\"{}\",password:\"{}\"}})
                    }}",
                    remote_username, remote_password
                ),
                None,
                None,
                None,
            )
            .await?
            .recv()
            .await
            .ok_or("failed to login")??;
        #[derive(Deserialize)]
        struct Payload {
            login: String,
        }
        let session = serde_json::from_value::<Payload>(login_response.data)?.login;
        connection.abort();
        client = client.set_actors(Session {
            session: Some(session),
        });
        (fut, subscriber) = client.try_connect().await?;
        connection = tokio::spawn(fut);
        let mut stream = subscriber
            .subscribe(
                &format!(
                    "subscription streamNotes{{
                        streamNotes{{
                            id
                            uuid
                            lamportClock
                            createdAt
                            updatedAt
                            deletedAt
                            userId
                            title
                            content
                        }}
                    }}"
                ),
                None,
                None,
                None,
            )
            .await?;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct NoteStream {
            stream_notes: model::note::Model,
        }
        while let Some(Ok(remote_note)) = stream.recv().await {
            println!("{}", serde_json::to_string(&remote_note)?);
            let remote_note = serde_json::from_value::<NoteStream>(remote_note.data)?.stream_notes;
            let local_note = model::note::Entity::find()
                .filter(model::note::Column::Uuid.eq(remote_note.uuid.clone()))
                .one(&self.db)
                .await?;
            if let Some(local_note) = local_note {
                let update_note = async {
                    let mut updated_note: model::note::ActiveModel = local_note.clone().into();
                    if local_note.created_at != remote_note.created_at {
                        updated_note.created_at = Set(remote_note.created_at);
                    }
                    if local_note.updated_at != remote_note.updated_at {
                        updated_note.updated_at = Set(remote_note.updated_at);
                    }
                    if local_note.deleted_at != remote_note.deleted_at {
                        updated_note.deleted_at = Set(remote_note.deleted_at);
                    }
                    if local_note.title != remote_note.title {
                        updated_note.title = Set(remote_note.title);
                    }
                    if local_note.content != remote_note.content {
                        updated_note.content = Set(remote_note.content);
                    }
                    updated_note.lamport_clock = Set(remote_note.lamport_clock.clone());
                    updated_note.update(&self.db).await?;
                    Ok::<(), Error>(())
                };
                if local_note.lamport_clock < remote_note.lamport_clock {
                    update_note.await?;
                } else if local_note.lamport_clock == remote_note.lamport_clock
                    && local_note.updated_at < remote_note.updated_at
                {
                    update_note.await?;
                }
            } else {
                model::note::ActiveModel {
                    uuid: Set(remote_note.uuid.clone()),
                    lamport_clock: Set(remote_note.lamport_clock.clone()),
                    created_at: Set(remote_note.created_at.clone()),
                    updated_at: Set(remote_note.updated_at.clone()),
                    deleted_at: Set(remote_note.deleted_at.clone()),
                    user_id: Set(user_id),
                    title: Set(remote_note.title.clone()),
                    content: Set(remote_note.content.clone()),
                    ..Default::default()
                }
                .insert(&self.db)
                .await?;
            }
        }
        connection.abort();
        Ok(())
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
    async fn note() -> Result<(), Error> {
        let note = init().await?;
        let count_notes =
            || async { Ok::<usize, Error>(model::note::Entity::find().count(&note.db).await?) };
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

    async fn init() -> Result<NoteModule, Error> {
        let config = new_config_provider(Mode::UnitTest)?;
        let db = new_db_connection(config.clone()).await?;
        Ok(new_note_module(db.clone()))
    }
}
