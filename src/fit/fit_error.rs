/// Error thrown when fitting isn't possible
#[derive(thiserror::Error, Debug)]
#[error("Insufficient available width ({available_width})")]
pub struct InsufficientWidthError {
    pub available_width: usize,
}
