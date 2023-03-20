use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// A model mirroring SpendingProof entity from Ergo node REST API.
/// See `SpendingProof` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiSpendingProof {}
