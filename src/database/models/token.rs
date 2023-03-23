use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::{BlockId, Height, HexString};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Token {}
