use anyhow::{Ok, Result};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Row,
};

use crate::{
    common::{BlockId, Height},
    config::DatabaseSettings,
};

use self::models::header::Header;

pub mod models;

#[derive(Debug, Clone)]
pub struct Database {
    pub conn_pool: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub async fn new(settings: &DatabaseSettings) -> Result<Self> {
        let conn_options = PgConnectOptions::new()
            .host(&settings.host)
            .port(settings.port)
            .username(&settings.user)
            .password(&settings.password);
        let conn_pool = PgPoolOptions::new()
            .max_connections(1000)
            .connect_with(conn_options)
            .await
            .expect("Unable to connect to the database");

        Ok(Database { conn_pool })
    }

    pub async fn get_best_height(&self) -> Result<Option<Height>> {
        let row = sqlx::query(
            "select height from node_headers where main_chain = true order by height desc limit 1",
        )
        .fetch_optional(&self.conn_pool)
        .await?;

        match row {
            Some(r) => {
                let height: i32 = r.try_get("height")?;
                Ok(Some(height))
            }
            None => Ok(None),
        }
    }

    pub async fn get_header_by_block_id(&self, id: &BlockId) -> Result<Option<Header>> {
        Ok(sqlx::query_as::<_, Header>(
                "SELECT id, parent_id, version, height, n_bits, difficulty, timestamp, state_root, ad_proofs_root, transactions_root, extension_hash, miner_pk, w, n, d, votes, main_chain FROM node_headers WHERE id = ?"
            )
            .bind(&id.value)
            .fetch_optional(&self.conn_pool).await?)
    }

    pub async fn get_all_headers_by_height(&self, height: &Height) -> Result<Vec<Header>> {
        Ok(sqlx::query_as::<_, Header>(
                "SELECT id, parent_id, version, height, n_bits, difficulty, timestamp, state_root, ad_proofs_root, transactions_root, extension_hash, miner_pk, w, n, d, votes, main_chain FROM node_headers WHERE height = ?"
            )
            .bind(&height)
            .fetch_all(&self.conn_pool).await?)
    }
}
