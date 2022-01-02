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
    Other(#[from] anyhow::Error)
}