use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, Address};

/// Represents `blocks_info` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct BlockStats {
    pub header_id: BlockId,
    pub timestamp: i64,
    pub height: Height,
    // pub difficulty: BigDecimal,
    pub block_size: i32,
    pub block_coins: i64,
    pub block_mining_time: Option<i64>,
    pub txs_count: i32,
    pub txn_size: i32,
    pub miner_address: Address,
    pub miner_reward: i64,
    pub miner_revenue: i64,
    pub block_fee: i64,
    pub block_chain_total_size: i64,
    pub total_txs_count: i64,
    pub total_coins_issued: i64,
    pub total_mining_time: i64,
    pub total_fees: i64,
    pub total_miners_reward: i64,
    pub total_coins_in_txs: i64,
    pub max_tx_gix: i64,
    pub max_box_gix: i64,
    pub main_chain: bool
}
