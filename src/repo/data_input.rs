use std::rc::Rc;

use anyhow::Result;

use crate::database::{models::data_input::DataInput, Database};

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
}
