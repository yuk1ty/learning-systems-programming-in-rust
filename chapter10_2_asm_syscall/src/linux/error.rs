use std::{fmt::Display, io};

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// `io::Error::last_os_error()` の返り値を内部に持つ。
#[derive(Debug)]
pub(crate) struct Error(io::Error);

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(&self.0)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self(e)
    }
}
