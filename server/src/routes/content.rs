use futures::{Stream, StreamExt};
use warp::Filter;

use crate::args::Args;

pub fn content(
    args: &Args,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(args.content_path.to_owned()).and(
        warp::post()
            .and(warp::body::stream())
            .and(warp::path::tail())
            .and_then(upload)
            .or(warp::fs::dir(args.content_dir.to_owned())),
    )
}

async fn upload<S, B>(body: S, path: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection>
where
    S: StreamExt,
    S: Stream<Item = Result<B, warp::Error>>,
    B: warp::Buf,
{
    Ok(warp::reply())
}
