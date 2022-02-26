use std::sync::PoisonError;

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse url: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("client unauthorized: {0}")]
    Unauthorized(String),

    #[error("gitlab error: {0}")]
    GitlabError(String),

    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("{0}")]
    VarError(#[from] std::env::VarError),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::Other(err.to_string())
    }
}
