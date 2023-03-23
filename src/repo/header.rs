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
        todo!()
    }

    pub async fn insert(&self, header: &Header) -> Result<()> {
        todo!()
    }
}
