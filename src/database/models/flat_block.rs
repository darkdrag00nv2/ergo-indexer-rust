use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{Address, BlockId, Height};

use super::{header::Header, block_stats::BlockStats};

/// Flattened representation of a full block from Ergo protocol enriched with statistics.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct FlatBlock {
    pub header: Header,
    pub info: BlockStats,
}
