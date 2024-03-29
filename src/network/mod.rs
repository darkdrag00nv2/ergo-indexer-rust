use std::time::Duration;

use crate::common::{BlockId, Height};
use anyhow::Result;
use rand::seq::SliceRandom;

use jsonrpsee::core::{client::ClientT, rpc_params};
use jsonrpsee_http_client::{HttpClient, HttpClientBuilder};

pub mod models;

use self::models::{api_full_block::ApiFullBlock, api_node_info::ApiNodeInfo};

/// A service providing an access to the Ergo network.
#[derive(Debug, Clone)]
pub struct ErgoLiveNetwork {
    pub base_url: String,

    /// This is a vector to support load balancing although for now we'll only have one node. If we
    /// want to support load balancing, then we need to take multiple networks in the .env.
    pub clients: Vec<HttpClient>,
}

impl ErgoLiveNetwork {
    pub fn new(base_url: String) -> Self {
        let mut clients = vec![];
        let timeout = Duration::from_secs(60);

        let client = HttpClientBuilder::default()
            .max_concurrent_requests(100000)
            .request_timeout(timeout)
            .build(&base_url)
            .unwrap();
        clients.push(client);

        ErgoLiveNetwork { base_url, clients }
    }

    pub async fn get_best_height(&self) -> Result<Height> {
        let client = self.get_client();

        let raw_info = client.request("/info", rpc_params![]).await?;
        let block = serde_json::from_value::<ApiNodeInfo>(raw_info)?;

        Ok(block.full_height)
    }

    pub async fn get_block_ids_at_height(&self, height: Height) -> Result<Vec<BlockId>> {
        todo!()
    }

    pub async fn get_full_block_by_id(&self, id: BlockId) -> Result<Option<ApiFullBlock>> {
        todo!()
    }

    /// Returns a random client out the client pool so that we can load balance between different
    /// Ergo nodes. At this point there is only a single node.
    fn get_client(&self) -> &HttpClient {
        let client = self.clients.choose(&mut rand::thread_rng()).unwrap();
        return client;
    }
}
