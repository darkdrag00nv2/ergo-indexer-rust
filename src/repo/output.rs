use std::rc::Rc;

use anyhow::Result;

use crate::{common::{Height, BlockId}, database::{Database, models::{header::Header, transaction::Transaction, output::Output}}};

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
}
