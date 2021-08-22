use std::fmt;
use std::io;

use serde_json as json;

pub mod adapter;
pub mod dap_type;
pub mod header;

#[derive(Debug)]
pub enum Error {
    /// The adapter receive a malformed message
    BadMessage,
    Io(io::Error),
    /// The adapter receive a well form, but invalid message (e.g a request without a command field)
    InvalidMessage,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<json::Error> for Error {
    fn from(err: json::Error) -> Error {
        match err.classify() {
            json::error::Category::Io => io::Error::new(io::ErrorKind::Other, err).into(),
            json::error::Category::Syntax => Error::BadMessage,
            json::error::Category::Data => Error::InvalidMessage,
            json::error::Category::Eof => Error::BadMessage,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadMessage => f.write_str("bad message"),
            Error::Io(err) => err.fmt(f),
            Error::InvalidMessage => f.write_str("invalid message"),
        }
    }
}

impl std::error::Error for Error {}
