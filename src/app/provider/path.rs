use std::path::PathBuf;

use dioxus::prelude::*;

use crate::app::component::{error, loading};

/// All files/directories used in this app must have the paths defined here.
#[derive(Clone, PartialEq, Eq)]
pub struct Paths {
    /// root dir of all data files/directories
    pub data_dir: PathBuf,
    /// path of the database's data file/directory
    pub db_path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    AppPathsError(#[from] app_path::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl Paths {
    fn create(data_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            db_path: data_dir.join("db").join("main.db"),
            data_dir,
        })
    }
}

#[inline_props]
pub fn path_provider<'a>(cx: Scope, children: Element<'a>) -> Element {
    let fut = use_future(&cx, (), |_| async {
        Ok::<Paths, Error>(Paths::create(app_path::get_data_dir()?)?)
    });
    cx.render(match fut.value() {
        None => {
            rsx!(loading::loading {"initializing data directory..."})
        }
        Some(Ok(paths)) => {
            use_context_provider(&cx, || paths.to_owned());
            rsx!(children)
        }
        Some(Err(e)) => rsx!(error::error { e.to_string() }),
    })
}
