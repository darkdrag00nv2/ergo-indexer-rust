use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{asset::Asset, header::Header, transaction::Transaction},
        Database,
    },
};

pub struct AssetRepo {
    db: Rc<Database>,
}

impl AssetRepo {
    pub fn new(db: Rc<Database>) -> Self {
        AssetRepo { db }
    }

    pub async fn insert_many(&self, txs: &Vec<Asset>) -> Result<()> {
        todo!()
    }
}
