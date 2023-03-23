use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, HexString};

/// Represents `node_outputs` table.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Output {}
