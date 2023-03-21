use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Represents a hex string. Must always be initialized with HexString::new.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HexString {
    pub value: String,
}

impl HexString {
    pub fn new(value: String) -> Self {
        // TODO: validate the format.
        HexString { value }
    }
}

pub type Height = i32;
pub type BlockId = HexString;
pub type TokenId = HexString;
pub type BoxId = String;
pub type TxId = String;

#[derive(Error, Debug)]
pub enum ErgoIndexerError {
    #[error("zero blocks written for height: {0}")]
    ZeroBlocksWritten(Height),
}
