use warp::{Filter, Reply};

use crate::schema::Schema;

use self::{playground::playground, query_mutation::query_mutation, subscription::subscription};

mod middlewares;
mod playground;
mod query_mutation;
mod subscription;

pub fn routes(
    schema: Schema,
) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path::end().and(
        playground()
            .or(query_mutation(schema.clone()))
            .or(subscription(schema)),
    )
}
