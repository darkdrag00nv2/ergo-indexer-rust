use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{header::Header, script_constant::ScriptConstant, transaction::Transaction},
        Database,
    },
};

pub struct ScriptConstantsRepo {
    db: Rc<Database>,
}

impl ScriptConstantsRepo {
    pub fn new(db: Rc<Database>) -> Self {
        ScriptConstantsRepo { db }
    }

    pub async fn insert_many(&self, constants: &Vec<ScriptConstant>) -> Result<()> {
        todo!()
    }
}
