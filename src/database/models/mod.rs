/// Represents a hex string. Must always be initialized with HexString::new.
pub struct HexString {
    pub value: String,
}

impl HexString {
    pub fn new(value: String) -> Self {
        // TODO: validate the format.
        HexString { value }
    }
}

pub type BlockId = HexString;
pub type TokenId = HexString;
pub type BoxId = String;

/// Represents `node_ad_proofs` table.
pub struct AdProof {
    pub header_id: BlockId,
    pub proof_of_bytes: HexString, // serialized and hex-encoded AVL+ tree path
    pub digest: HexString,         // hex-encoded tree root hash
}
