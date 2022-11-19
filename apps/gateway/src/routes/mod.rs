use warp::{Filter, Rejection, Reply};

use crate::{schema::Schema, Gateway};

use self::{api::create_api_route, error::handle_error, health::create_health_route};

mod api;
mod error;
mod health;
mod middlewares;

pub fn create_routes(
    schema: Schema,
    nb: &Gateway,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api_route = warp::path(nb.config.api_path.clone())
        .and(warp::path::end())
        .and(create_api_route(schema, nb));
    let health_route = warp::path(nb.config.health_path.clone()).and(create_health_route());

    api_route.or(health_route).recover(handle_error)
}
