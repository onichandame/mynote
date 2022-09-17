use std::error::Error;

use ::tracing::{debug, trace};
use args::Args;
use async_graphql::{extensions, Schema};
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use resolver::{Mutation, Query, Subscription};
use sea_orm::Database;
use warp::Filter;

use crate::{extension::CurrentUser, tracing::setup_trace};

mod args;
mod auth;
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
    let args = Args::parse();
    args.validate()?;
    let db = Database::connect(&args.database_url).await?;
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

    let app = routes::create_routes(schema.clone(), &args, &db);
    let app = app.with(warp::trace::request());
    let cors = warp::cors()
        .allow_methods(vec!["POST", "GET"])
        .allow_headers(["Cache-Control", "Content-Type", "Pragma", "Authorization"]);
    let cors = if args.allow_origins.len() > 0 {
        cors.allow_origins(args.allow_origins.iter().map(AsRef::as_ref))
    } else {
        cors.allow_any_origin()
    };
    let app = app.with(cors);
    warp::serve(app)
        .run(([0, 0, 0, 0], args.port.clone()))
        .await;

    trace!("server shutting down");
    Ok(())
}
