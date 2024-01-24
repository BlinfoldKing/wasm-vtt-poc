use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum ErrorKind {
    Config,

    InternalError,
    NotFound,
    Unauthorized,
    Unauthenticated,
    BadRequest,

    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error {
    pub fn new(message: String, kind: ErrorKind) -> Error {
        tracing::error!(message);
        Error { message, kind }
    }
}
