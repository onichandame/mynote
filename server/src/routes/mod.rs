use warp::{Filter, Reply};

use crate::{args::Args, schema::Schema};

use self::{
    content::content, playground::playground, query_mutation::query_mutation,
    subscription::subscription,
};

mod content;
mod middlewares;
mod playground;
mod query_mutation;
mod subscription;

pub fn routes(
    schema: Schema,
    args: &Args,
) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    content(args).or(warp::path::end().and(
        playground()
            .or(query_mutation(schema.clone()))
            .or(subscription(schema)),
    ))
}
