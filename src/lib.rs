use std::io;
use thiserror::Error;

mod header;
mod message;

pub use header::*;
pub use message::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid input")]
    Invalid,
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    InvalidJson(#[from] serde_json::Error),
}
