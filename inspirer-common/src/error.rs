pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    LibraryLoadingError(#[from] libloading::Error),
    #[error("Error to load application.")]
    LoadApplicationError,
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}