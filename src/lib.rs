use config::ErgoIndexerConfig;
use network::ErgoNetwork;
use repo::RepoBundle;

pub mod config;
pub mod database;
pub mod network;
pub mod redis;
pub mod repo;

pub fn start_indexing(config: ErgoIndexerConfig) {}

pub struct ErgoIndexer {
    pub config: ErgoIndexerConfig,
    pub network: Box<dyn ErgoNetwork>,
    pub repos: RepoBundle,
}
