use async_graphql::Request;
use async_graphql_warp::{graphql, GraphQLResponse};
use warp::{Filter, Rejection, Reply};

use crate::schema::Schema;

pub fn query_mutation(
    schema: Schema,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post().and(graphql(schema.clone()).and_then(
        |(schema, request): (Schema, Request)| async move {
            Ok::<_, Rejection>(GraphQLResponse::from(schema.execute(request).await))
        },
    ))
}
