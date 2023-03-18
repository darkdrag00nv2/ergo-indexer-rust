mod defaults;

#[derive(Debug, Clone)]
pub struct NetworkSettings {

}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {

}

#[derive(Debug, Clone)]
pub struct RedisSettings {

}


#[derive(Debug, Clone)]
pub struct ErgoIndexerConfig {
    pub chain_poll_duration_secs: Option<i32>,
    pub network: NetworkSettings,
    pub db: DatabaseSettings,
    pub start_height: Option<u64>,
    pub redis: RedisSettings,
}

impl ErgoIndexerConfig {
    pub fn new() -> Self {
        todo!()
    }
}
