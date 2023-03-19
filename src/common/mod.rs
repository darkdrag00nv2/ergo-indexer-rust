use thiserror::Error;

pub type Height = u64;

#[derive(Error, Debug)]
pub enum ErgoIndexerError {
    #[error("zero blocks written for height: {0}")]
    ZeroBlocksWritten(Height),
}
