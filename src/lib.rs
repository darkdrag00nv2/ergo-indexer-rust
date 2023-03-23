use std::{sync::Arc, thread::sleep, time::Duration, vec};

use crate::common::{ErgoIndexerError, Height};
use anyhow::{bail, Result};
use async_recursion::async_recursion;
use common::BlockId;
use config::ErgoIndexerConfig;
use database::models::{block_stats::BlockStats, flat_block::FlatBlock, header::Header};
use log::info;
use network::{models::api_full_block::ApiFullBlock, ErgoLiveNetwork};
use repo::RepoBundle;

pub mod common;
pub mod config;
pub mod database;
pub mod network;
pub mod redis;
pub mod repo;

pub async fn start_indexing(config: ErgoIndexerConfig) -> Result<()> {
    let network = Arc::new(ErgoLiveNetwork::new(config.network.url.to_owned()));
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
    pub network: Arc<ErgoLiveNetwork>,
    pub repos: RepoBundle,
    start_height: Height,
}

impl ErgoIndexer {
    pub fn new(
        config: ErgoIndexerConfig,
        network: Arc<ErgoLiveNetwork>,
        repos: RepoBundle,
    ) -> Self {
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
        let block_count = ids.len();
        info!("Grabbing blocks at height {}: {:#?}", height, ids);

        let full_block_tasks = ids.clone().into_iter().map(|id| {
            let network = Arc::clone(&self.network);
            tokio::spawn(async move {
                let network = Arc::clone(&network);
                network.get_full_block_by_id(id).await
            })
        });
        let mut blocks = vec![];
        for fbt in full_block_tasks {
            blocks.push(fbt.await??);
        }

        let mut best_block = vec![];
        let mut orphaned_blocks = vec![];
        for block in blocks {
            match block {
                Some(block) => match ids.first() {
                    Some(id) => {
                        if id.value.contains(&block.header.id.value) {
                            best_block.push(block);
                        } else {
                            orphaned_blocks.push(block);
                        }
                    }
                    None => continue,
                },
                None => bail!(ErgoIndexerError::BlockNotFoundForId(BlockId::new(
                    String::from("TODO")
                ))),
            }
        }

        info!("Best block is {:#?}", best_block);
        info!("Orphaned blocks are {:#?}", orphaned_blocks);
        debug_assert!(
            best_block.len() == 1,
            "{}",
            format!(
                "Exactly one best block was expected for height {}, blocks: {:#?}",
                height, best_block
            )
        );

        self.apply_best_block(&best_block[0]).await?;
        Ok(block_count as i32)
    }

    // TODO: This Send bound might be needed to make sure that the returned future can be sent
    // between threads. Right now, we have removed it.
    #[async_recursion(?Send)]
    async fn apply_best_block(&self, block: &ApiFullBlock) -> Result<()> {
        let id = &block.header.id;
        let height = block.header.height;
        let parent_id = &block.header.parent_id;
        let parent_height = block.header.height - 1;

        match self.get_block(parent_id).await? {
            Some(parent_block) if parent_block.main_chain => {}
            None if block.header.height == self.start_height => {}
            Some(parent_block) => {
                info!("Parent block {} needs to be updated", parent_id);
                self.update_best_block(parent_block).await?;
            }
            None => {
                info!("Parent block {} needs to be downloaded", parent_id);
                let parent_block = self.network.get_full_block_by_id(parent_id.clone()).await?;
                match parent_block {
                    Some(parent_block) => self.apply_best_block(&parent_block).await?,
                    None => bail!(ErgoIndexerError::InconsistentNodeView(
                        parent_id.clone(),
                        parent_height
                    )),
                }
            }
        }

        let parent_info = self.get_block_info(parent_id).await?;
        let flat_block = self.scan(block, parent_info).await?;
        self.insert_block(&flat_block).await?;
        self.mark_as_main(id, height).await
    }

    async fn update_best_block(&self, parent_block: Header) -> Result<()> {
        todo!()
    }

    async fn insert_block(&self, block: &FlatBlock) -> Result<()> {
        let insertion_tasks = tokio::join!(
            self.repos.headers.insert(&block.header),
            self.repos.txs.insert_many(&block.txs),
            self.repos.inputs.insert_many(&block.inputs),
            self.repos.data_inputs.insert_many(&block.data_inputs),
        );
        insertion_tasks.0?;
        insertion_tasks.1?;
        insertion_tasks.2?;
        insertion_tasks.3?;
        Ok(())
    }

    async fn mark_as_main(&self, id: &BlockId, height: Height) -> Result<()> {
        todo!()
    }

    async fn scan(
        &self,
        api_full_block: &ApiFullBlock,
        prev_block_info_opt: Option<BlockStats>,
    ) -> Result<FlatBlock> {
        // FlatBlock {
        //     header: todo!(),
        //     info: todo!(),
        // }
        todo!()
    }

    async fn get_block(&self, id: &BlockId) -> Result<Option<Header>> {
        self.repos.headers.get(id).await
    }

    async fn get_block_info(&self, id: &BlockId) -> Result<Option<BlockStats>> {
        self.repos.blocks_info.get(id).await
    }
}
