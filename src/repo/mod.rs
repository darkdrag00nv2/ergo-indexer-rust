use crate::{database::Database, config::DatabaseSettings};
use anyhow::Result;

pub struct RepoBundle {
    database: Database,
}

impl RepoBundle {
    pub async fn new(db_settings: &DatabaseSettings) -> Result<Self> {
        let database = Database::new(db_settings).await?;

        Ok(RepoBundle { database })
    }
}
