use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, HexString};

/// Represents `node_assets` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Asset {}
