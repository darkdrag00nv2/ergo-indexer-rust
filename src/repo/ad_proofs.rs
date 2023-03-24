use std::rc::Rc;

use anyhow::Result;

use crate::database::{models::ad_proof::AdProof, Database};

pub struct AdProofRepo {
    db: Rc<Database>,
}

impl AdProofRepo {
    pub fn new(db: Rc<Database>) -> Self {
        AdProofRepo { db }
    }

    pub async fn insert(&self, proof: &AdProof) -> Result<()> {
        todo!()
    }
}
