use std::rc::Rc;

use anyhow::Result;

use crate::{common::{Height, BlockId}, database::{Database, models::header::Header}};

pub struct HeaderRepo {
    db: Rc<Database>,
}

impl HeaderRepo {
    pub fn new(db: Rc<Database>) -> Self {
        HeaderRepo { db }
    }

    pub async fn get_best_height(&self) -> Result<Option<Height>> {
        self.db.get_best_height().await
    }

    pub async fn get(&self, id: &BlockId) -> Result<Option<Header>> {
        self.db.get_header_by_block_id(id).await
    }

    pub async fn get_all_by_height(&self, height: &Height) -> Result<Vec<Header>> {
        self.db.get_all_headers_by_height(height).await
    }

    pub async fn insert(&self, header: &Header) -> Result<()> {
        todo!()
    }

    pub async fn update_chain_status_by_id(&self, id: &BlockId, new_chain_status: bool) -> Result<()> {
        todo!()
    }
}
