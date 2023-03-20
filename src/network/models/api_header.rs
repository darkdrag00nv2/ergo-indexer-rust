use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, HexString};

pub type ApiDifficulty = BigDecimal;

/// A model mirroring `BlockHeader` entity from Ergo node REST API.
/// See `BlockHeader` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
/// TODO: Implement custom serializer and deserializer for the struct.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiHeader {
    pub id: BlockId,
    pub parent_id: BlockId,
    pub version: u8,
    pub height: i32,
    pub n_bits: i64,
    // pub difficulty: ApiDifficulty,
    pub timestamp: i64,
    pub state_root: HexString,
    pub ad_proofs_root: HexString,
    pub transactions_root: HexString,
    pub extension_hash: HexString,
    pub miner_pk: HexString,
    pub w: HexString,
    pub n: HexString,
    pub d: String,
    pub votes: String,
}
