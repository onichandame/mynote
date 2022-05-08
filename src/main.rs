use ::session::new_session_module;
use async_graphql::Schema;
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use config::{new_config_provider, Mode};
use db::new_db_connection;
use frontend::Frontend;
use resolver::{Mutation, Query, Subscription};
use serde::Deserialize;
use std::{error::Error, net::SocketAddr};
use tokio;
use warp::{ws::Ws, Filter};
use warp_embed;

use crate::session::Session;

mod auth;
mod frontend;
mod helper;
mod note;
mod password;
mod resolver;
mod session;
mod sync;
mod user;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::init();
    // build schema
    let config = new_config_provider(Mode::Production)?;
    let db = new_db_connection(config.clone()).await?;
    let session = new_session_module(db.clone());
    let schema = Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(config.clone())
    .data(session.clone())
    .data(db.clone())
    .finish();

    // api route
    let apis = warp::path!("graphql")
        .and(warp::ws())
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .map(move |ws: Ws, schema, protocol| {
            let reply = ws.on_upgrade(move |sock| {
                GraphQLWebSocket::new(sock, schema, protocol)
                    .on_connection_init(|v| async {
                        #[derive(Deserialize)]
                        struct Payload {
                            session: Session,
                        }
                        let mut data = async_graphql::Data::default();
                        if let Ok(payload) = serde_json::from_value::<Payload>(v) {
                            data.insert(payload.session);
                        }
                        Ok(data)
                    })
                    .serve()
            });
            warp::reply::with_header(
                reply,
                "Sec-Websocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        });
    // webpages
    let pages = warp::get().and(warp_embed::embed(&Frontend));

    let app = apis.or(pages);

    warp::serve(app)
        .run(config.server_addr.parse::<SocketAddr>()?)
        .await;
    Ok(())
}
