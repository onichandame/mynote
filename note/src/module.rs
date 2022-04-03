use std::error::Error as StdError;

use async_trait::async_trait;
use crud::{Count, Create, Delete, Get, List, Stream, Update, DB};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tokio_graphql_ws::{Client, ClientActor};

use crate::{NoteFilter, NoteSorting};

type Error = Box<dyn StdError + Send + Sync>;

#[derive(Clone)]
pub struct NoteModule {
    db: DatabaseConnection,
}

/// constructor
pub fn new_note_module(db: DatabaseConnection) -> NoteModule {
    NoteModule { db }
}

impl DB for NoteModule {
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}

impl List<model::note::Entity, NoteFilter, Vec<NoteSorting>> for NoteModule {}

impl Count<model::note::Entity, NoteFilter> for NoteModule {}

impl Get<model::note::Entity, NoteFilter> for NoteModule {}

impl Create<model::note::ActiveModel> for NoteModule {}

impl Update<model::note::ActiveModel, NoteFilter> for NoteModule {}

impl Delete<model::note::Entity, NoteFilter> for NoteModule {}

impl Stream<model::note::Entity, NoteFilter> for NoteModule {}

impl NoteModule {
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

#[cfg(test)]
mod tests {
    use config::{new_config_provider, Mode};
    use crud::{Filter, Pagination, SortDirection};
    use db::new_db_connection;
    use sea_orm::PaginatorTrait;

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
        let created_note = note
            .create(model::note::Insert {
                user_id: user.id,
                title: "".to_owned(),
                content: "".to_owned(),
                ..Default::default()
            })
            .await?;
        assert_eq!(1, count_notes().await?);
        // get
        let got_note = note
            .get(&NoteFilter {
                id: Some(Filter {
                    eq: Some(created_note.id),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await?;
        assert_eq!(created_note.id, got_note.id);
        // list
        let listed_notes = note
            .list(
                &Default::default(),
                &Default::default(),
                &Default::default(),
            )
            .await?;
        assert_eq!(1, listed_notes.len());
        assert_eq!(
            created_note.id,
            listed_notes.get(0).ok_or("fatal error")?.id
        );
        // count
        assert_eq!(1, note.count(&Default::default()).await?);
        // filter
        assert_eq!(
            0,
            note.count(&NoteFilter {
                id: Some(Filter {
                    lt: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .await?
        );
        // pagination
        assert_eq!(
            0,
            note.list(
                &Default::default(),
                &Pagination {
                    limit: Some(0),
                    ..Default::default()
                },
                &Default::default()
            )
            .await?
            .len()
        );
        assert_eq!(
            0,
            note.list(
                &Default::default(),
                &Pagination {
                    limit: Some(1),
                    offset: Some(1),
                    ..Default::default()
                },
                &Default::default()
            )
            .await?
            .len()
        );
        // sorting
        let second_note = note
            .create(model::note::Insert {
                user_id: user.id,
                title: "".to_owned(),
                content: "".to_owned(),
                ..Default::default()
            })
            .await?;
        assert_eq!(
            created_note.id,
            note.list(
                &Default::default(),
                &Default::default(),
                &vec!(NoteSorting {
                    field: model::note::Column::Id,
                    direction: SortDirection::ASC
                })
            )
            .await?
            .get(0)
            .ok_or("fatal error")?
            .id
        );
        assert_eq!(
            second_note.id,
            note.list(
                &Default::default(),
                &Default::default(),
                &vec!(NoteSorting {
                    field: model::note::Column::Id,
                    direction: SortDirection::DESC
                })
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
                &NoteFilter {
                    id: Some(Filter {
                        eq: Some(created_note.id),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                model::note::Update {
                    title: Some(title.clone()),
                    ..Default::default()
                }
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
            .delete(&NoteFilter {
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
