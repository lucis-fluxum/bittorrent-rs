use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid metadata: {0}")]
    InvalidMetadata(String),
    #[error("invalid socket address: {0}:{1}")]
    InvalidSocketAddress(String, u16),
}
