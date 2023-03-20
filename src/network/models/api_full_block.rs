use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::api_header::ApiHeader;

/// A model mirroring `FullBlock` entity from Ergo node REST API.
/// See `FullBlock` in https://github.com/ergoplatform/ergo/blob/master/src/main/resources/api/openapi.yaml
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct ApiFullBlock {
    pub header: ApiHeader,
    pub size: i32
}
