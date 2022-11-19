use serde::{Serialize, Serializer};
use warp::{hyper::StatusCode, Rejection, Reply};

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    #[serde(serialize_with = "serialize_status_code")]
    code: StatusCode,
    message: String,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "UNHANDLED_REJECTION".to_owned(),
        }
    }
}

impl<T: ToString> From<T> for Error {
    fn from(s: T) -> Self {
        Self::with_message(&s.to_string())
    }
}

impl Error {
    pub fn with_message(msg: &str) -> Self {
        Self {
            message: msg.to_owned(),
            ..Default::default()
        }
    }
}

impl warp::reject::Reject for Error {}

pub async fn handle_error(e: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(err) = e.find::<Error>() {
        Ok(warp::reply::with_status(warp::reply::json(&err), err.code))
    } else {
        Err(e)
    }
}

fn serialize_status_code<S>(code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(code.as_u16())
}
