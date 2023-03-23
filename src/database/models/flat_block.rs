use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{Address, BlockId, Height};

use super::{block_stats::BlockStats, header::Header, input::Input, transaction::Transaction, data_input::DataInput};

/// Flattened representation of a full block from Ergo protocol enriched with statistics.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct FlatBlock {
    pub header: Header,
    pub info: BlockStats,
    pub txs: Vec<Transaction>,
    pub inputs: Vec<Input>,
    pub data_inputs: Vec<DataInput>,
}
