use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::BlockId,
    database::{models::block_stats::BlockStats, Database},
};

pub struct BlockInfoRepo {
    db: Rc<Database>,
}

impl BlockInfoRepo {
    pub fn new(db: Rc<Database>) -> Self {
        BlockInfoRepo { db }
    }

    pub async fn get(&self, id: &BlockId) -> Result<Option<BlockStats>> {
        todo!()
    }

    pub async fn insert(&self, block_info: &BlockStats) -> Result<()> {
        todo!()
    }

    pub async fn update_chain_status_by_header_id(
        &self,
        header_id: &BlockId,
        new_chain_status: bool,
    ) -> Result<()> {
        self.db
            .update_chain_status_by_header_id("blocks_info", header_id, new_chain_status)
            .await
    }
}
