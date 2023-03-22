use crate::common::{BlockId, HexString};

pub mod header;
pub mod block_stats;
pub mod flat_block;

/// Represents `node_ad_proofs` table.
pub struct AdProof {
    pub header_id: BlockId,
    pub proof_of_bytes: HexString, // serialized and hex-encoded AVL+ tree path
    pub digest: HexString,         // hex-encoded tree root hash
}
