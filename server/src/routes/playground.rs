use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use warp::{http, Filter, Rejection, Reply};

pub fn playground() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get().map(|| {
        http::Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
            ))
    })
}
