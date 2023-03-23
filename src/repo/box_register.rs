use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{box_register::BoxRegister, header::Header, transaction::Transaction},
        Database,
    },
};

pub struct BoxRegisterRepo {
    db: Rc<Database>,
}

impl BoxRegisterRepo {
    pub fn new(db: Rc<Database>) -> Self {
        BoxRegisterRepo { db }
    }

    pub async fn insert_many(&self, registers: &Vec<BoxRegister>) -> Result<()> {
        todo!()
    }
}
