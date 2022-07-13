use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection};

pub fn extract_session(db: DatabaseConnection) -> impl Filter + Clone {
    warp::header::optional::<String>("authorization")
        .and_then(|token| async move { Ok::<String, Rejection>("".to_owned()) })
}
