use std::convert::Infallible;

use warp::{Filter, Rejection};

use crate::{auth::session::Session, Gateway};

pub fn extract_session(
    nb: &Gateway,
) -> impl Filter<Extract = (Option<Session>,), Error = Rejection> + Clone {
    let nb = nb.clone();
    warp::header::optional::<String>("Authorization").and_then(move |token: Option<String>| {
        let nb = nb.clone();
        async move {
            let nb = nb.clone();
            if let Some(token) = token {
                if let Some(token) = token.strip_prefix("Bearer ") {
                    return Ok::<_, Infallible>(nb.auth.session.parse_from_token(token).await.ok());
                }
            }
            Ok(None)
        }
    })
}
