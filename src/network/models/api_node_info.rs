use crate::common::{BlockId, HexString};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::api_node_info_epoch_parameters::ApiNodeInfoEpochParameters;

/// A model mirroring `NodeInfo` entity from Ergo node REST API.
/// See `NodeInfo` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiNodeInfo {
    pub current_time: i64,
    pub name: String,
    pub state_type: String,
    pub difficulty: i64,
    pub best_full_header_id: BlockId,
    pub best_header_id: BlockId,
    pub peers_count: i32,
    pub unconfirmed_count: i32,
    pub app_version: String,
    pub state_root: HexString,
    pub previous_full_header_id: BlockId,
    pub full_height: i32,
    pub headers_height: i32,
    pub state_version: HexString,
    pub launch_time: i64,
    pub parameters: ApiNodeInfoEpochParameters,
    pub is_mining: bool
}
