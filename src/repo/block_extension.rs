use std::rc::Rc;

use anyhow::Result;

use crate::database::{models::block_extension::BlockExtension, Database};

pub struct BlockExtensionRepo {
    db: Rc<Database>,
}

impl BlockExtensionRepo {
    pub fn new(db: Rc<Database>) -> Self {
        BlockExtensionRepo { db }
    }

    pub async fn insert(&self, extension: &BlockExtension) -> Result<()> {
        todo!()
    }
}
