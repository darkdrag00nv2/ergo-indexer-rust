use std::env;

use dotenv::dotenv;
use ergo_indexer_rust::{config, start_indexing};

fn main() {
    dotenv().ok();

    log::info!("Reading the indexer config");
    let indexer_config = config::read_ergo_indexer_config(env::vars());
    log::info!("The indexer config is {:#?}", indexer_config);

    start_indexing(indexer_config);
}
