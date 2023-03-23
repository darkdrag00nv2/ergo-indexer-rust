use std::rc::Rc;

use crate::{config::DatabaseSettings, database::Database};
use anyhow::Result;

use self::{
    block_info::BlockInfoRepo, data_input::DataInputRepo, header::HeaderRepo, input::InputRepo,
    transaction::TransactionRepo,
};

mod block_info;
mod data_input;
mod header;
mod input;
mod transaction;

pub struct RepoBundle {
    pub database: Rc<Database>,
    pub headers: HeaderRepo,
    pub blocks_info: BlockInfoRepo,
    pub txs: TransactionRepo,
    pub inputs: InputRepo,
    pub data_inputs: DataInputRepo,
}

impl RepoBundle {
    pub async fn new(db_settings: &DatabaseSettings) -> Result<Self> {
        let database = Rc::new(Database::new(db_settings).await?);

        Ok(RepoBundle {
            database: database.clone(),
            headers: HeaderRepo::new(database.clone()),
            blocks_info: BlockInfoRepo::new(database.clone()),
            txs: TransactionRepo::new(database.clone()),
            inputs: InputRepo::new(database.clone()),
            data_inputs: DataInputRepo::new(database.clone()),
        })
    }
}
