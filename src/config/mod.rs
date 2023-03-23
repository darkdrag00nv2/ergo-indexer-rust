use crate::common::{Address, Height};
use std::fmt::Debug;
use std::{collections::HashMap, env::Vars};

mod defaults;

#[derive(Debug, Clone)]
pub struct NetworkSettings {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RedisSettings {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct ProtocolSettings {
    pub network_prefix: String,
    pub genesis_address: Address,
}

#[derive(Debug, Clone)]
pub struct ErgoIndexerConfig {
    pub chain_poll_duration_secs: u64,
    pub network: NetworkSettings,
    pub db: DatabaseSettings,
    pub protocol: ProtocolSettings,
    pub start_height: Height,
    pub redis: RedisSettings,
}

pub fn read_ergo_indexer_config(vars: Vars) -> ErgoIndexerConfig {
    let mut envs = HashMap::new();
    for (key, value) in vars {
        envs.insert(key, value);
    }

    ErgoIndexerConfig {
        chain_poll_duration_secs: str::parse::<u64>(
            envs.get("CHAIN_POLL_DURATION_SECS").unwrap().as_str(),
        )
        .unwrap(),
        network: NetworkSettings {
            url: envs.get("NETWORK_URL").unwrap().to_string(),
        },
        protocol: ProtocolSettings {
            network_prefix: envs.get("NETWORK_PREFIX").unwrap().to_string(),
            genesis_address: Address::new(envs.get("NETWORK_PREFIX").unwrap().to_string()),
        },
        db: DatabaseSettings {
            host: envs.get("DATABASE_HOST").unwrap().to_string(),
            port: str::parse::<u16>(envs.get("DATABASE_PORT").unwrap().as_str()).unwrap(),
            user: envs.get("DATABASE_USER").unwrap().to_string(),
            password: envs.get("DATABASE_PWD").unwrap().to_string(),
        },
        start_height: str::parse::<i32>(envs.get("START_HEIGHT").unwrap().as_str()).unwrap(),
        redis: RedisSettings {
            url: envs.get("REDIS_URL").unwrap().to_string(),
        },
    }
}
