/// Termimad error type
///
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Crossterm error: {0}")]
    Crossterm(#[from] crossterm::ErrorKind),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
