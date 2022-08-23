use async_graphql::{Error, ServerError};

pub fn error_to_server_error(e: Error) -> ServerError {
    e.into_server_error((0, 0).into())
}
