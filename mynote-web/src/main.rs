use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use frontend::Frontend;
use mynote_core::MyNote;
use resolver::{Mutation, Query};
use serde::Deserialize;
use std::env;
use tokio;
use warp::{ws::Ws, Filter};
use warp_embed;

use crate::session::Session;

mod conversion;
mod dto;
mod frontend;
mod guard;
mod resolver;
mod session;

#[tokio::main]
pub async fn main() {
    let core = MyNote::create().await.unwrap();
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(core.clone())
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
        })
        .or(warp_embed::embed(&Frontend));
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        _other => 80,
    };
    warp::serve(app).run(([0, 0, 0, 0], port)).await;
}
