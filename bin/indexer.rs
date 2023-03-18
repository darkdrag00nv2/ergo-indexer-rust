use std::env;

use dotenv::dotenv;
use ergo_indexer_rust::config;

fn main() {
    dotenv().ok();

    let indexer_config = config::read_ergo_indexer_config(env::vars());
    println!("The indexer config is {:#?}", indexer_config);
}
