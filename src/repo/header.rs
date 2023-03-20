use std::rc::Rc;

use anyhow::Result;

use crate::{common::Height, database::Database};

pub struct HeaderRepo {
    db: Rc<Database>
}

impl HeaderRepo {
    pub fn new(db: Rc<Database>) -> Self {
        HeaderRepo {
            db
        }
    }

    pub async fn get_best_height(&self) -> Result<Option<Height>> {
        self.db.get_best_height().await
    }
}
