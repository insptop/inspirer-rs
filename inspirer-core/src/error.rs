use std::borrow::Cow;

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Serialize;

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
    ConfigError(#[from] config::ConfigError),
    #[error("Failed to read config.")]
    ReadConfigError,
    #[error("Failed to load or write config.")]
    LoadOrWriteConfigError,
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Get configuration failed.")]
    GetConfigurationFailedError,
    #[error(transparent)]
    BizError(#[from] BizError),
    #[error("Internal system error.")]
    ExtractApplicationComponentError,
    #[error("Internal system error.")]
    ComponentManagerError,
    #[error("Unknown system error.")]
    UnknownError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
pub struct BizError {
    pub status: StatusCode,
    pub code: String,
    pub msg: String,
}

impl BizError {
    pub fn create_report(code: &str, msg: &str, status: StatusCode) -> Error {
        Error::BizError(BizError {
            code: code.into(),
            msg: msg.into(),
            status
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    code: String,
    msg: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::BizError(BizError { status, code, msg }) => {
                (status, Json(ErrorMessage { code, msg })).into_response()
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
