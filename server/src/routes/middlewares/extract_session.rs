use warp::{Filter, Rejection};

use crate::auth::Session;

pub fn extract_session() -> impl Filter<Extract = (Option<Session>,), Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization").and_then(|token: Option<String>| async move {
        Ok::<Option<Session>, Rejection>(
            token
                .and_then(|token| token.strip_prefix("Bearer ").map(|v| v.to_owned()))
                .map(|v| Session(v)),
        )
    })
}
