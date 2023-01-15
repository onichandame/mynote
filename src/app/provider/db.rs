use dioxus::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use super::path::Paths;

pub type Db = DatabaseConnection;

pub enum Error {}

/// The only database instance in this app is provided by this provider. All
/// persistent data should be stored in this database
#[inline_props]
pub fn db_provider<'a>(cx: Scope, children: Element<'a>) -> Element {
    let paths = use_context::<Paths>(&cx);
    //let fut = use_future(&cx, (paths,), |(paths)| async move {
    //    Database::connect(
    //        ConnectOptions::new("sqlite:/data/data/com.example.notebook/db.sqlite".to_owned())
    //            .min_connections(1)
    //            .to_owned(),
    //    )
    //    .await
    //});
    cx.render(rsx! {
        children
    })
}
