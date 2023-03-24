use std::rc::Rc;

use anyhow::Result;

use crate::{database::{models::data_input::DataInput, Database}, common::BlockId};

pub struct DataInputRepo {
    db: Rc<Database>,
}

impl DataInputRepo {
    pub fn new(db: Rc<Database>) -> Self {
        DataInputRepo { db }
    }

    pub async fn insert_many(&self, txs: &Vec<DataInput>) -> Result<()> {
        todo!()
    }

    pub async fn update_chain_status_by_header_id(&self, header_id: &BlockId, new_chain_status: bool) -> Result<()> {
        todo!()
    }
}
