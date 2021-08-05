use std::fmt;
use std::io;

pub mod header;

#[derive(Debug)]
pub enum Error {
    /// The adapter receive a malformed message
    BadMessage,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadMessage => f.write_str("bad message"),
            Error::Io(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
