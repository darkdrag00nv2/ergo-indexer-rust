use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{header::Header, output::Output, transaction::Transaction},
        Database,
    },
};

pub struct OutputRepo {
    db: Rc<Database>,
}

impl OutputRepo {
    pub fn new(db: Rc<Database>) -> Self {
        OutputRepo { db }
    }

    pub async fn insert_many(&self, outputs: &Vec<Output>) -> Result<()> {
        todo!()
    }

    pub async fn update_chain_status_by_header_id(
        &self,
        header_id: &BlockId,
        new_chain_status: bool,
    ) -> Result<()> {
        self.db
            .update_chain_status_by_header_id("node_outputs", header_id, new_chain_status)
            .await
    }
}
