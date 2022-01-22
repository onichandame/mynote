use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("The requested not found or not accessible by the user")]
    NoteNotFound,
}
