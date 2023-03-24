use std::rc::Rc;

use crate::{config::DatabaseSettings, database::Database};
use anyhow::Result;

use self::{
    asset::AssetRepo, block_extension::BlockExtensionRepo, block_info::BlockInfoRepo,
    box_register::BoxRegisterRepo, data_input::DataInputRepo, header::HeaderRepo, input::InputRepo,
    output::OutputRepo, script_constants::ScriptConstantsRepo, token::TokenRepo,
    transaction::TransactionRepo, ad_proofs::AdProofRepo,
};

mod ad_proofs;
mod asset;
mod block_extension;
mod block_info;
mod box_register;
mod data_input;
mod header;
mod input;
mod output;
mod script_constants;
mod token;
mod transaction;

pub struct RepoBundle {
    pub database: Rc<Database>,
    pub headers: HeaderRepo,
    pub blocks_info: BlockInfoRepo,
    pub block_extensions: BlockExtensionRepo,
    pub ad_proofs: AdProofRepo,
    pub txs: TransactionRepo,
    pub inputs: InputRepo,
    pub data_inputs: DataInputRepo,
    pub outputs: OutputRepo,
    pub assets: AssetRepo,
    pub registers: BoxRegisterRepo,
    pub tokens: TokenRepo,
    pub constants: ScriptConstantsRepo,
}

impl RepoBundle {
    pub async fn new(db_settings: &DatabaseSettings) -> Result<Self> {
        let database = Rc::new(Database::new(db_settings).await?);

        Ok(RepoBundle {
            database: database.clone(),
            headers: HeaderRepo::new(database.clone()),
            blocks_info: BlockInfoRepo::new(database.clone()),
            block_extensions: BlockExtensionRepo::new(database.clone()),
            ad_proofs: AdProofRepo::new(database.clone()),
            txs: TransactionRepo::new(database.clone()),
            inputs: InputRepo::new(database.clone()),
            data_inputs: DataInputRepo::new(database.clone()),
            outputs: OutputRepo::new(database.clone()),
            assets: AssetRepo::new(database.clone()),
            registers: BoxRegisterRepo::new(database.clone()),
            tokens: TokenRepo::new(database.clone()),
            constants: ScriptConstantsRepo::new(database.clone()),
        })
    }
}
