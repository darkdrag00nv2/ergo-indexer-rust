use core::fmt;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

/// Represents a hex string. Must always be initialized with HexString::new.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, FromRow)]
pub struct HexString {
    pub value: String,
}

impl fmt::Display for HexString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<std::string::String> for HexString {
    fn from(value: std::string::String) -> Self {
        HexString::new(value)
    }
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
pub type BlockIdWithHeight = (BlockId, Height);

/// Represents an address. Must always be initialized with Address::new.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    pub value: String,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<std::string::String> for Address {
    fn from(value: std::string::String) -> Self {
        Address::new(value)
    }
}

impl Address {
    pub fn new(value: String) -> Self {
        // TODO: validate the format.
        Address { value }
    }
}

#[derive(Error, Debug)]
pub enum ErgoIndexerError {
    #[error("Zero blocks written for height: {0}")]
    ZeroBlocksWritten(Height),
    #[error("Block not found for id: {0}")]
    BlockNotFoundForId(BlockId),
    #[error("Failed to pull best block with id: {0} and height: {1}")]
    InconsistentNodeView(BlockId, Height),
}
