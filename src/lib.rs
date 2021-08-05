use thiserror::Error;
use std::io;

mod header;

pub use header::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid input")]
    Invalid,
    #[error("{0}")]
    Io(#[from] io::Error),
}