use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};

use crate::{args::Args, schema::Schema};

use self::{api::create_api, content::create_content, error::handle_error};

mod api;
mod content;
mod error;
mod middlewares;

pub fn create_routes(
    schema: Schema,
    args: &Args,
    db: &DatabaseConnection,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let api_route = warp::path(args.api_path.to_owned())
        .and(warp::path::end())
        .and(create_api(schema, db));
    let content_route =
        warp::path(args.content_path.to_owned()).and(create_content(&args.content_dir, db));

    api_route.or(content_route).recover(handle_error)
}
