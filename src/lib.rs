use std::{thread::sleep, time::Duration};

use crate::common::{ErgoIndexerError, Height};
use anyhow::{bail, Result};
use common::BlockId;
use config::ErgoIndexerConfig;
use log::info;
use network::ErgoLiveNetwork;
use repo::RepoBundle;

pub mod common;
pub mod config;
pub mod database;
pub mod network;
pub mod redis;
pub mod repo;

pub async fn start_indexing(config: ErgoIndexerConfig) -> Result<()> {
    let network = ErgoLiveNetwork::new(config.network.url.to_owned());
    let repos = RepoBundle::new(&config.db)
        .await
        .expect("Couldn't create RepoBundle.");

    // Save poll duration before config is moved into ErgoIndexer.
    let chain_poll_duration_secs = config.chain_poll_duration_secs;
    let indexer = ErgoIndexer::new(config, network, repos);

    info!("Starting indexer sync...");
    loop {
        indexer.sync().await.expect("Unexpected error in sync.");
        // TODO: We are not really polling every chain_poll_duration_secs. Instead we are polling
        // every processing time + chain_poll_duration_secs. Consider having a separate thread for
        // polling so that indexing loop doesn't have to wait for receiving the network block height.
        sleep(Duration::from_secs(chain_poll_duration_secs));
    }
}

pub struct ErgoIndexer {
    pub config: ErgoIndexerConfig,
    pub network: ErgoLiveNetwork,
    pub repos: RepoBundle,
    start_height: Height,
}

impl ErgoIndexer {
    pub fn new(config: ErgoIndexerConfig, network: ErgoLiveNetwork, repos: RepoBundle) -> Self {
        // TODO: Make the start_height optional and use genesis height as default.
        let start_height = config.start_height;

        ErgoIndexer {
            config,
            network,
            repos,
            start_height,
        }
    }

    pub async fn sync(&self) -> Result<()> {
        let (network_height, local_height) = tokio::join!(
            self.network.get_best_height(),
            self.get_last_grabbed_block_height()
        );
        let network_height = network_height?;
        let local_height = local_height?;
        info!(
            "Current network height: {:#?}, Current indexer height: {:#?}",
            network_height, local_height
        );

        for height in (local_height + 1)..(network_height + 1) {
            let num_blocks_grabbed = self.perform_indexing_at_height(height as Height).await?;
            match num_blocks_grabbed {
                0 => {
                    bail!(ErgoIndexerError::ZeroBlocksWritten(height))
                }
                _ => info!(
                    "{} block(s) grabbed from height {}",
                    num_blocks_grabbed, height
                ),
            }
        }

        Ok(())
    }

    async fn get_last_grabbed_block_height(&self) -> Result<Height> {
        let stored_in_db = match self.repos.headers.get_best_height().await? {
            Some(h) => h,
            None => self.start_height,
        };
        Ok(std::cmp::max(stored_in_db, self.start_height - 1))
    }

    async fn perform_indexing_at_height(&self, height: Height) -> Result<i32> {
        let ids: Vec<BlockId> = self.network.get_block_ids_at_height(height).await?;
        info!("Grabbing blocks at height {}: {:#?}", height, ids);

        // Why we have to move the id inside the async closure: https://stackoverflow.com/a/63437482.
        let full_block_tasks = ids.into_iter().map(|id| {});

        Ok(0)
    }
}
