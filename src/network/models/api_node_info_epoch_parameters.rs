use crate::common::Height;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiNodeInfoEpochParameters {
    pub height: Height,
    pub storage_fee_factor: i32,
    pub min_value_per_byte: i32,
    pub max_block_size: i32,
    pub max_block_cost: i32,
    pub block_version: u8,
    pub token_access_cost: i32,
    pub input_cost: i32,
    pub data_input_cost: i32,
    pub output_cost: i32,
}
