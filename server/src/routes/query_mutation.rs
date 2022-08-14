use async_graphql::Request;
use async_graphql_warp::{graphql, GraphQLResponse};
use warp::{Filter, Rejection, Reply};

use crate::schema::Schema;

use super::middlewares::extract_session;

pub fn query_mutation(
    schema: Schema,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(extract_session())
        .and(graphql(schema.clone()))
        .and_then(
            |token, (schema, mut request): (Schema, Request)| async move {
                if let Some(session) = token {
                    request = request.data(session);
                }
                Ok::<_, Rejection>(GraphQLResponse::from(schema.execute(request).await))
            },
        )
}
