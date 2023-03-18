use anyhow::{Ok, Result};
use config::ErgoIndexerConfig;
use network::{ErgoLiveNetwork, ErgoNetwork};
use repo::RepoBundle;

pub mod config;
pub mod database;
pub mod network;
pub mod redis;
pub mod repo;

pub async fn start_indexing(config: ErgoIndexerConfig) -> Result<()> {
    // TODO: need to pass network details to the network so that it can connect to the node.
    let network = Box::new(ErgoLiveNetwork::new());
    let repos = RepoBundle::new(&config.db)
        .await
        .expect("Couldn't create RepoBundle.");

    let indexer = ErgoIndexer::new(config, network, repos);
    loop {
        indexer.sync().await.expect("Unexpected error in sync.");
    }
}

pub struct ErgoIndexer {
    pub config: ErgoIndexerConfig,
    pub network: Box<dyn ErgoNetwork>,
    pub repos: RepoBundle,
}

impl ErgoIndexer {
    pub fn new(
        config: ErgoIndexerConfig,
        network: Box<dyn ErgoNetwork>,
        repos: RepoBundle,
    ) -> Self {
        ErgoIndexer {
            config,
            network,
            repos,
        }
    }

    pub async fn sync(&self) -> Result<()> {
        todo!()
    }
}
