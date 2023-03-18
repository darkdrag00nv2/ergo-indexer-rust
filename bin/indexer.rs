use std::env;

use dotenv::dotenv;
use ergo_indexer_rust::{config, start_indexing};
use log::*;
use simple_logger::SimpleLogger;

#[tokio::main()]
async fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    info!("Reading the indexer config");
    let indexer_config = config::read_ergo_indexer_config(env::vars());
    info!("The indexer config is {:#?}", indexer_config);

    start_indexing(indexer_config)
        .await
        .expect("Couldn't start indexing operation.");
}
