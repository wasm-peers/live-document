use thiserror::Error;

pub type LiveDocumentResult<T> = Result<T, LiveDocumentError>;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum LiveDocumentError {
    #[error("missing element: {0}")]
    MissingElement(String),
    #[error("unexpected element: {0}")]
    UnexpectedElement(String),
    #[error("failed to create URL search params: {0}")]
    FailedToCreateUrlSearchParams(String),
}
