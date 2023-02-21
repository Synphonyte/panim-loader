use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse error: {0}")]
    ParseError(#[from] nom::error::Error<String>),
    #[error("Error opening file: {0}")]
    FileError(#[from] std::io::Error),
}