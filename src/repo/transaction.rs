use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{header::Header, transaction::Transaction},
        Database,
    },
};

pub struct TransactionRepo {
    db: Rc<Database>,
}

impl TransactionRepo {
    pub fn new(db: Rc<Database>) -> Self {
        TransactionRepo { db }
    }

    pub async fn insert_many(&self, txs: &Vec<Transaction>) -> Result<()> {
        todo!()
    }

    pub async fn update_chain_status_by_header_id(
        &self,
        header_id: &BlockId,
        new_chain_status: bool,
    ) -> Result<()> {
        todo!()
    }
}
