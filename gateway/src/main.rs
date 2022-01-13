use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use auth;
use db;
use resolver::{Mutation, Query};
use serde::Deserialize;
use std::env;
use tokio;
use warp::{ws::Ws, Filter};
use warp_embed;
use web;

mod resolver;

#[tokio::main]
pub async fn main() {
    let db_connection_pool = db::new_connection_pool().await;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_connection_pool.clone())
        .finish();
    let app = warp::path!("graphql")
        .and(warp::ws())
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .map(move |ws: Ws, schema, protocol| {
            let reply = ws.on_upgrade(move |sock| {
                GraphQLWebSocket::new(sock, schema, protocol)
                    .on_connection_init(|v| async {
                        #[derive(Deserialize)]
                        struct Payload {
                            session: auth::Session,
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
        })
        .or(warp_embed::embed(&web::Frontend));
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        _other => 80,
    };
    warp::serve(app).run(([0, 0, 0, 0], port)).await;
}
