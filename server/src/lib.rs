use std::error::Error;

use ::tracing::{debug, trace};
use async_graphql::{extensions, Schema};
use clap::Parser;
use config::Config;
use migration::{Migrator, MigratorTrait};
use resolver::{Mutation, Query, Subscription};
use sea_orm::Database;
use tokio::fs;
use warp::Filter;

use crate::{extension::CurrentUser, tracing::setup_trace};

mod auth;
mod config;
mod entity;
mod extension;
mod migration;
mod resolver;
mod routes;
mod schema;
mod tracing;

pub async fn start_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    setup_trace().await?;
    trace!("server starting");
    let config = Config::parse();
    config.validate()?;
    let db = Database::connect(&config.database_url).await?;
    Migrator::up(&db, None).await?;
    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(db.clone())
    .extension(CurrentUser)
    .extension(extensions::Tracing)
    .finish();
    debug!(schema = schema.sdl(), "graphql schema built");
    fs::create_dir_all(&config.content_dir).await?;

    let app = routes::create_routes(schema.clone(), &config, &db);
    let app = app.with(warp::trace::request());
    let cors = warp::cors()
        .allow_methods(vec!["POST", "GET"])
        .allow_headers(["Cache-Control", "Content-Type", "Pragma", "Authorization"]);
    let cors = if config.allow_origins.len() > 0 {
        cors.allow_origins(config.allow_origins.iter().map(AsRef::as_ref))
    } else {
        cors.allow_any_origin()
    };
    let app = app.with(cors);
    warp::serve(app)
        .run(([0, 0, 0, 0], config.port.clone()))
        .await;

    trace!("server shutting down");
    Ok(())
}
