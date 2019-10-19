/// Termimad error type
///
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Crossterm error")]
    Crossterm(#[from] crossterm::ErrorKind),
    #[error("IO error")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
