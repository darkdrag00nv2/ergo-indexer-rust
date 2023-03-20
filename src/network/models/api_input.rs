use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::BoxId;

use super::api_spending_proof::ApiSpendingProof;

/// A model mirroring ErgoTransactionInput entity from Ergo node REST API.
/// See `ErgoTransactionInput` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiInput {
    pub box_id: BoxId,
    pub spending_proof: ApiSpendingProof,
}
