use std::error::Error;

use ::tracing::{debug, trace};
use args::Args;
use async_graphql::{extensions, Schema};
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use resolver::{Mutation, Query, Subscription};
use sea_orm::Database;
use warp::Filter;

use crate::tracing::setup_trace;

mod args;
mod auth;
mod entity;
mod migration;
mod resolver;
mod routes;
mod schema;
mod tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    setup_trace().await?;
    trace!("server starting");
    let args = Args::parse();
    let db = Database::connect(&args.database_url).await?;
    Migrator::up(&db, None).await?;
    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(db.clone())
    .extension(extensions::Tracing)
    .finish();
    debug!(schema = schema.sdl(), "graphql schema built");

    let app = routes::routes(schema.clone());
    let app = app.with(warp::trace::request());
    let cors = warp::cors();
    let cors = if args.allow_origins.len() > 0 {
        cors.allow_origins(args.allow_origins.iter().map(AsRef::as_ref))
    } else {
        cors.allow_any_origin()
    };
    let app = app.with(cors);
    warp::serve(app).run(([0, 0, 0, 0], args.port)).await;

    trace!("server shutting down");
    Ok(())
}
