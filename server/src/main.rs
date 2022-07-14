use std::{error::Error, net::SocketAddr};

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

    warp::serve(routes::routes(schema.clone()).with(warp::trace::request()))
        .run(args.addr.parse::<SocketAddr>()?)
        .await;

    trace!("server shutting down");
    Ok(())
}
