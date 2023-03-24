use std::{sync::Arc, thread::sleep, time::Duration, vec};

use crate::common::{ErgoIndexerError, Height};
use anyhow::{bail, Result};
use async_recursion::async_recursion;
use common::{BlockId, BlockIdWithHeight};
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
    let mut indexer = ErgoIndexer::new(config, network, repos);

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
    config: ErgoIndexerConfig,
    network: Arc<ErgoLiveNetwork>,
    repos: RepoBundle,
    start_height: Height,
    pending_chain_updates: Vec<BlockIdWithHeight>,
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
            // TODO: Do we need a mutex? Most likely not.
            pending_chain_updates: Vec::new(),
        }
    }

    pub async fn sync(&mut self) -> Result<()> {
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

    async fn perform_indexing_at_height(&mut self, height: Height) -> Result<i32> {
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
        self.commit_chain_updates().await?;
        Ok(block_count as i32)
    }

    // TODO: This Send bound might be needed to make sure that the returned future can be sent
    // between threads. Right now, we have removed it.
    #[async_recursion(?Send)]
    async fn apply_best_block(&mut self, block: &ApiFullBlock) -> Result<()> {
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
        Ok(self.mark_as_main(id, height).await)
    }

    // TODO: Consider using spawn or join for optional writes as well. Could be done using an empty
    // task for false cases.
    async fn insert_block(&self, block: &FlatBlock) -> Result<()> {
        let insertion_tasks = tokio::join!(
            self.repos.headers.insert(&block.header),
            self.repos.txs.insert_many(&block.txs),
            self.repos.inputs.insert_many(&block.inputs),
            self.repos.data_inputs.insert_many(&block.data_inputs),
            self.repos.outputs.insert_many(&block.outputs),
            self.repos.assets.insert_many(&block.assets),
            self.repos.tokens.insert_many(&block.tokens),
        );
        if self.config.indexes.block_stats {
            self.repos.blocks_info.insert(&block.info).await?
        }
        if self.config.indexes.block_extensions {
            self.repos.block_extensions.insert(&block.extension).await?
        }
        if self.config.indexes.ad_proofs {
            if let Some(ad_proof) = &block.ad_proof_opt {
                self.repos.ad_proofs.insert(ad_proof).await?
            }
        }
        if self.config.indexes.box_registers {
            self.repos.registers.insert_many(&block.registers).await?
        }
        if self.config.indexes.box_registers {
            self.repos.constants.insert_many(&block.constants).await?
        }

        insertion_tasks.0?;
        insertion_tasks.1?;
        insertion_tasks.2?;
        insertion_tasks.3?;
        insertion_tasks.4?;
        insertion_tasks.5?;
        insertion_tasks.6?;
        Ok(())
    }

    async fn mark_as_main(&mut self, id: &BlockId, height: Height) {
        self.pending_chain_updates.push((id.clone(), height));
    }

    async fn update_best_block(&self, parent_block: Header) -> Result<()> {
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

    async fn commit_chain_updates(&mut self) -> Result<()> {
        info!(
            "Updating {} chain slots: {:#?}",
            self.pending_chain_updates.len(),
            self.pending_chain_updates
        );

        for (id, height) in &self.pending_chain_updates {
            let blocks = self.get_header_ids_at_height(height).await?;
            let mut non_best_blocks = vec![];
            for block in &blocks {
                if block != id {
                    non_best_blocks.push(block);
                }
            }

            self.update_chain_status(id, true).await?;
            for non_best in non_best_blocks {
                self.update_chain_status(non_best, false).await?;
            }
        }

        self.pending_chain_updates.clear();
        Ok(())
    }

    async fn update_chain_status(&self, id: &BlockId, is_main_chain: bool) -> Result<()> {
        todo!()
    }

    async fn get_block(&self, id: &BlockId) -> Result<Option<Header>> {
        self.repos.headers.get(id).await
    }

    async fn get_header_ids_at_height(&self, height: &Height) -> Result<Vec<BlockId>> {
        todo!()
    }

    async fn get_block_info(&self, id: &BlockId) -> Result<Option<BlockStats>> {
        self.repos.blocks_info.get(id).await
    }
}
