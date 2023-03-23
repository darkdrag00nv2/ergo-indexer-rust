use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, HexString};

/// Represents `node_ad_proofs` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct AdProof {
    pub header_id: BlockId,
    pub proof_of_bytes: HexString, // serialized and hex-encoded AVL+ tree path
    pub digest: HexString,         // hex-encoded tree root hash
}
