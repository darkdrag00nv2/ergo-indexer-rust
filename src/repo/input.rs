use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{models::{header::Header, input::Input}, Database},
};

pub struct InputRepo {
    db: Rc<Database>,
}

impl InputRepo {
    pub fn new(db: Rc<Database>) -> Self {
        InputRepo { db }
    }

    pub async fn insert_many(&self, inputs: &Vec<Input>) -> Result<()> {
        todo!()
    }
}
