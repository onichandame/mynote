use std::fs;

use dioxus::prelude::*;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use url::Url;

use crate::app::component::{error, loading};

use super::path::Paths;

pub type Db = DatabaseConnection;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
    #[error("{0}")]
    Unknown(String),
}

/// The only database instance in this app is provided by this provider. All
/// persistent data should be stored in this database
#[inline_props]
pub fn db_provider<'a>(cx: Scope, children: Element<'a>) -> Element {
    let paths = use_context::<Paths>(&cx).unwrap();
    let fut = use_future(&cx, (paths,), |(paths,)| async move {
        if let Some(dir) = paths.db_path.parent() {
            fs::create_dir_all(&dir)?;
        }
        let uri = format!(
            "sqlite:{}?mode=rwc",
            Url::from_file_path(paths.db_path)
                .map_err(|_| Error::Unknown("failed to retrieve db parent folder".to_owned()))?
                .to_string()
        );
        let db = Database::connect(ConnectOptions::new(uri).min_connections(1).to_owned()).await?;
        Migrator::up(&db, None).await?;
        Ok::<Db, Error>(db)
    });
    cx.render(match fut.value() {
        None => rsx!(loading::loading{"loading database..."}),
        Some(Ok(db)) => {
            cx.provide_context(db.to_owned());
            rsx!(children)
        }
        Some(Err(e)) => rsx!(error::error{e.to_string()}),
    })
}
