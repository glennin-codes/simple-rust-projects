use thiserror::Error;
 
#[derive(Error, Debug)]
pub enum FileError{
    #[error("An I/O error ocurred {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("No content found")]
    Empty,
    #[error("unknown File error occured")]
    Unknown,

 }