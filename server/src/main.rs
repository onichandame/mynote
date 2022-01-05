use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use db;
use resolver::{Mutation, Query};
use warp::{ws::Ws, Filter};

mod resolver;

#[tokio::main]
pub async fn main() {
    let db_connection_pool = db::new_connection_pool().await;
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db_connection_pool.clone())
        .extension(auth::Auth)
        .finish();
    let app = warp::path!("graphql")
        .and(warp::ws())
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .map(move |ws: Ws, schema, protocol| {
            let reply = ws.on_upgrade(move |sock| {
                GraphQLWebSocket::new(sock, schema, protocol)
                    .on_connection_init(auth::pass_session)
                    .serve()
            });
            warp::reply::with_header(
                reply,
                "Sec-Websocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        })
        .or(warp_embed::embed(&frontend::Spa));
    warp::serve(app).run(([0, 0, 0, 0], 80)).await;
}
