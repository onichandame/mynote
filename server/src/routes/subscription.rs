use async_graphql::{Data, Result};
use async_graphql_warp::{graphql_protocol, GraphQLWebSocket};
use serde::Deserialize;
use warp::{ws::Ws, Filter};

use crate::{auth::Session, schema::Schema};

use super::middlewares::extract_session;

pub fn subscription(
    schema: Schema,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::ws()
        .and(extract_session())
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .map(move |ws: Ws, session, schema, protocol| {
            let reply = ws.on_upgrade(move |sock| {
                let mut data = Data::default();
                data.insert(session);
                GraphQLWebSocket::new(sock, schema, protocol)
                    .with_data(data)
                    .on_connection_init(on_connection_init)
                    .serve()
            });
            warp::reply::with_header(
                reply,
                "Sec-WebSocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        })
}

async fn on_connection_init(v: serde_json::Value) -> Result<Data> {
    #[derive(Deserialize)]
    struct ConnectionInitPayload {
        authorization: Option<String>,
    }
    let mut data = Data::default();
    let payload = serde_json::from_value::<ConnectionInitPayload>(v)?;
    data.insert(payload.authorization.map(|v| Session(v)));
    Ok(data)
}
