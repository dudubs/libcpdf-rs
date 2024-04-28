use std::ffi::NulError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("{0:?}")]
    NulError(#[from] NulError),

    #[error("{0:?}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    Message(String),

    #[error("no pages to move")]
    NoPagesToMove,
}
