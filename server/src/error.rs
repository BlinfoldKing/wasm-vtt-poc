use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    Config,
    Http,

    InternalError,

    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error {
    pub fn new(message: String, kind: ErrorKind) -> Error {
        Error { message, kind }
    }
}
