use anyhow::Result;
use config::ErgoIndexerConfig;
use network::ErgoLiveNetwork;
use repo::RepoBundle;

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

    let indexer = ErgoIndexer::new(config, network, repos);
    loop {
        indexer.sync().await.expect("Unexpected error in sync.");
    }
}

pub struct ErgoIndexer {
    pub config: ErgoIndexerConfig,
    pub network: ErgoLiveNetwork,
    pub repos: RepoBundle,
}

impl ErgoIndexer {
    pub fn new(config: ErgoIndexerConfig, network: ErgoLiveNetwork, repos: RepoBundle) -> Self {
        ErgoIndexer {
            config,
            network,
            repos,
        }
    }

    pub async fn sync(&self) -> Result<()> {
        todo!();
    }
}
