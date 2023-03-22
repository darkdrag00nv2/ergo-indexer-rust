use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, HexString};

/// Represents `node_headers` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Header {
    pub id: BlockId,
    pub parent_id: BlockId,
    pub version: u8,
    pub height: Height,
    pub n_bits: i64,
    // pub difficulty: BigDecimal,
    pub timestamp: i64,
    pub state_root: HexString,
    pub ad_proofs_root: HexString,
    pub transaction_root: HexString,
    pub extension_hash: HexString,
    pub miner_pk: HexString,
    pub w: HexString,
    pub n: HexString,
    pub d: String,
    pub votes: String,
    pub main_chain: bool,
}
