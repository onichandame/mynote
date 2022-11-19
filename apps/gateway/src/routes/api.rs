use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data, Request,
};
use async_graphql_warp::{graphql, graphql_protocol, GraphQLResponse, GraphQLWebSocket};
use serde::Deserialize;
use warp::{http, ws::Ws, Filter, Rejection, Reply};

use crate::{schema::Schema, Gateway};

use super::middlewares::extract_session;

pub fn create_api_route(
    schema: Schema,
    nb: &Gateway,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let nb = nb.clone();
    let query_mutation = warp::post()
        .and(extract_session(&nb))
        .and(graphql(schema.clone()))
        .and_then(
            |session, (schema, mut request): (Schema, Request)| async move {
                if let Some(session) = session {
                    request = request.data(session);
                }
                Ok::<_, Rejection>(GraphQLResponse::from(schema.execute(request).await))
            },
        );
    let subscription = warp::ws()
        .and(extract_session(&nb))
        .and(warp::any().map(move || schema.clone()))
        .and(graphql_protocol())
        .and_then(move |ws: Ws, session, schema, protocol| {
            let nb = nb.clone();
            async move {
                let nb = nb.clone();
                let reply = ws.on_upgrade(move |sock| {
                    let nb = nb.clone();
                    let mut data = Data::default();
                    if let Some(session) = session {
                        data.insert(session);
                    }
                    GraphQLWebSocket::new(sock, schema, protocol)
                        .with_data(data)
                        .on_connection_init(|v: serde_json::Value| async move {
                            let nb = nb.clone();
                            let mut data = Data::default();
                            #[derive(Deserialize)]
                            struct ConnectionInitPayload {
                                authorization: String,
                            }
                            if let Ok(payload) = serde_json::from_value::<ConnectionInitPayload>(v)
                            {
                                if let Ok(session) = nb
                                    .auth
                                    .session
                                    .parse_from_token(&payload.authorization)
                                    .await
                                {
                                    data.insert(session);
                                }
                            }
                            Ok(data)
                        })
                        .serve()
                });
                Ok::<_, Rejection>(warp::reply::with_header(
                    reply,
                    "Sec-WebSocket-Protocol",
                    protocol.sec_websocket_protocol(),
                ))
            }
        });
    let playground = warp::get().map(|| {
        http::Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("").subscription_endpoint(""),
            ))
    });
    query_mutation.or(subscription).or(playground)
}
