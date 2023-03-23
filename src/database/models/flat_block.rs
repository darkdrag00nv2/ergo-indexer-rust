use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{
    ad_proof::AdProof, asset::Asset, block_extension::BlockExtension, block_stats::BlockStats,
    box_register::BoxRegister, data_input::DataInput, header::Header, input::Input, output::Output,
    script_constant::ScriptConstant, token::Token, transaction::Transaction,
};

/// Flattened representation of a full block from Ergo protocol enriched with statistics.
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct FlatBlock {
    pub header: Header,
    pub info: BlockStats,
    pub extension: BlockExtension,
    pub ad_proof_opt: Option<AdProof>,
    pub txs: Vec<Transaction>,
    pub inputs: Vec<Input>,
    pub data_inputs: Vec<DataInput>,
    pub outputs: Vec<Output>,
    pub assets: Vec<Asset>,
    pub registers: Vec<BoxRegister>,
    pub tokens: Vec<Token>,
    pub constants: Vec<ScriptConstant>,
}
