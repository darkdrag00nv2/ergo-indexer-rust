use std::rc::Rc;

use anyhow::Result;

use crate::{
    common::{BlockId, Height},
    database::{
        models::{header::Header, token::Token, transaction::Transaction},
        Database,
    },
};

pub struct TokenRepo {
    db: Rc<Database>,
}

impl TokenRepo {
    pub fn new(db: Rc<Database>) -> Self {
        TokenRepo { db }
    }

    pub async fn insert_many(&self, tokens: &Vec<Token>) -> Result<()> {
        todo!()
    }
}
