use axum::response::IntoResponse;
use hyper::StatusCode;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    LibraryLoadingError(#[from] libloading::Error),
    #[error("Error to load application.")]
    LoadApplicationError,
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error("Runtime build error: {0}")]
    RuntimeBuildError(#[source] std::io::Error),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}