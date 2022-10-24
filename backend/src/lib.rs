use ::tracing::{debug, trace};
use async_graphql::{extensions, Schema};
use auth::AuthModule;
use clap::Parser;
use config::Config;
use migration::Migrator;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use warp::Filter;

use crate::{
    extension::CurrentUser,
    resolver::{Mutation, Query, Subscription},
};

mod auth;
mod config;
mod entity;
mod extension;
mod migration;
mod resolver;
mod routes;
mod schema;

#[derive(Clone)]
pub struct Notebook {
    pub db: DatabaseConnection,
    pub config: Config,
    pub auth: AuthModule,
}

impl Notebook {
    pub async fn create() -> anyhow::Result<Self> {
        let config = Config::try_parse()?;
        config.validate()?;
        let db = Database::connect(
            ConnectOptions::new(config.database_url.clone())
                .min_connections(1)
                .to_owned(),
        )
        .await?;
        Migrator::up(&db, None).await?;
        let auth = AuthModule::new(db.clone(), config.clone());
        Ok(Self { db, config, auth })
    }

    /// start a blocking server
    pub async fn start_server(&self) -> anyhow::Result<()> {
        trace!("server starting");
        let schema = Schema::build(
            Query::default(),
            Mutation::default(),
            Subscription::default(),
        )
        .data(self.clone())
        .data(self.db.clone())
        .extension(CurrentUser)
        .extension(extensions::Tracing)
        .finish();
        debug!(schema = schema.sdl(), "graphql schema built");

        let app = routes::create_routes(schema.clone(), self);
        let app = app.with(warp::trace::request());
        let cors = warp::cors()
            .allow_methods(vec!["POST", "GET"])
            .allow_headers(["Cache-Control", "Content-Type", "Pragma", "Authorization"]);
        let cors = if self.config.allow_origins.len() > 0 {
            cors.allow_origins(self.config.allow_origins.iter().map(AsRef::as_ref))
        } else {
            cors.allow_any_origin()
        };
        let app = app.with(cors);
        warp::serve(app)
            .run(([0, 0, 0, 0], self.config.port.clone()))
            .await;

        trace!("server shutting down");
        Ok(())
    }
}
