use std::convert::Infallible;

use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection};

use crate::auth::session::Session;

pub fn extract_session(
    db: &DatabaseConnection,
) -> impl Filter<Extract = (Option<Session>,), Error = Rejection> + Clone {
    let db = db.clone();
    warp::header::optional::<String>("Authorization").and_then(move |token: Option<String>| {
        let db = db.clone();
        async move {
            if let Some(token) = token {
                if let Some(token) = token.strip_prefix("Bearer ") {
                    return Ok::<_, Infallible>(Session::try_from_token(token, &db).await.ok());
                }
            }
            Ok(None)
        }
    })
}
