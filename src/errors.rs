use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use edgedb_errors::display::display_error_verbose;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum PageError {
    #[error(transparent)]
    EdgeDBQueryError(#[from] edgedb_errors::Error),
    #[error("Other error: {0}")]
    Other(String),
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        tracing::debug!("To convert PageError: {:?}", self);
        let (status, message) = match self {
            Self::EdgeDBQueryError(ref e) => {
                tracing::info!("EdgeDB error: {}", display_error_verbose(e));
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e)
        };
        (status, message).into_response()
    }
}
