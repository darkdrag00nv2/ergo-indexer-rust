use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{models::header::Header, Database},
};

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
        self.db.insert_header(header).await
    }

    pub async fn update_chain_status_by_id(
        &self,
        id: &BlockId,
        new_chain_status: bool,
    ) -> Result<()> {
        self.db
            .update_chain_status_by_id(id, new_chain_status)
            .await
    }
}
