use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::BlockId;

use super::api_transaction::ApiTransaction;

/// A model mirroring BlockTransactions entity from Ergo node REST API.
/// See `BlockTransactions` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiBlockTransactions {
    pub header_id: BlockId,
    pub transactions: Vec<ApiTransaction>,
}
