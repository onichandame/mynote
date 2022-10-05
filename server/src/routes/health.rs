use warp::Filter;

pub fn create_health_route(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(move || async { Ok::<_, warp::Rejection>(warp::reply::reply()) })
}
