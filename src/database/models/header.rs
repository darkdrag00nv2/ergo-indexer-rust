use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, HexString};

/// Represents `node_headers` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Header {
    #[sqlx(try_from = "String")]
    pub id: BlockId,
    #[sqlx(try_from = "String")]
    pub parent_id: BlockId,
    pub version: i16,
    pub height: Height,
    pub n_bits: i64,
    // pub difficulty: BigDecimal,
    pub timestamp: i64,
    #[sqlx(try_from = "String")]
    pub state_root: HexString,
    #[sqlx(try_from = "String")]
    pub ad_proofs_root: HexString,
    #[sqlx(try_from = "String")]
    pub transaction_root: HexString,
    #[sqlx(try_from = "String")]
    pub extension_hash: HexString,
    #[sqlx(try_from = "String")]
    pub miner_pk: HexString,
    #[sqlx(try_from = "String")]
    pub w: HexString,
    #[sqlx(try_from = "String")]
    pub n: HexString,
    pub d: String,
    pub votes: String,
    pub main_chain: bool,
}
