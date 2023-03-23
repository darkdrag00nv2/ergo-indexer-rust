use std::rc::Rc;

use anyhow::Result;

use crate::{database::{Database, models::{block_stats::BlockStats}}, common::BlockId};

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
}