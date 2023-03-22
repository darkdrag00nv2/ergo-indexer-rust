use std::rc::Rc;

use crate::{config::DatabaseSettings, database::Database};
use anyhow::Result;

use self::{header::HeaderRepo, block_info::BlockInfoRepo};

mod header;
mod block_info;

pub struct RepoBundle {
    pub database: Rc<Database>,
    pub headers: HeaderRepo,
    pub blocks_info: BlockInfoRepo,
}

impl RepoBundle {
    pub async fn new(db_settings: &DatabaseSettings) -> Result<Self> {
        let database = Rc::new(Database::new(db_settings).await?);

        Ok(RepoBundle {
            database: database.clone(),
            headers: HeaderRepo::new(database.clone()),
            blocks_info: BlockInfoRepo::new(database.clone()),
        })
    }
}
