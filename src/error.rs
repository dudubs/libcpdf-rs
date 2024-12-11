use std::{
    ffi::NulError,
    ops::{FromResidual, Try},
};

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

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
