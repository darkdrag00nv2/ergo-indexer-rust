use std::rc::Rc;

use anyhow::Result;

use crate::{common::{Height, BlockId}, database::{Database, models::{header::Header, transaction::Transaction}}};

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
}
