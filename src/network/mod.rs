use anyhow::Result;
use crate::common::Height;

/// A service providing an access to the Ergo network.
pub struct ErgoLiveNetwork {
    pub base_url: String,
}

impl ErgoLiveNetwork {
    pub fn new(base_url: String) -> Self {
        ErgoLiveNetwork {
            base_url
        }
    }

    pub async fn get_best_height(&self) -> Result<Height> {
        todo!()
    }
}
