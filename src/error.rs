use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("{0}")]
    InvalidOption(String),

    #[error("No available port")]
    NoAvailablePort,
}

pub type Result<T> = std::result::Result<T, Errors>;
