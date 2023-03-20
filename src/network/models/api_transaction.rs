use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::TxId;

use super::{api_data_input::ApiDataInput, api_input::ApiInput, api_output::ApiOutput};

/// A model mirroring ErgoTransaction entity from Ergo node REST API.
/// See `ErgoTransaction` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiTransaction {
    pub id: TxId,
    pub inputs: Vec<ApiInput>,
    pub data_inputs: Vec<ApiDataInput>,
    pub outputs: Vec<ApiOutput>,
    pub size: i32,
}
