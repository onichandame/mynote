use async_graphql_warp::graphql_subscription;
use warp::Filter;

use crate::schema::Schema;

pub fn subscription(
    schema: Schema,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    graphql_subscription(schema.clone())
}
