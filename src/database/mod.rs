use anyhow::{Result, Ok};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, Row};

use crate::{common::Height, config::DatabaseSettings};

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
        .fetch_optional(&self.conn_pool).await?;

        match row {
            Some(r) => {
                let height: i32 = r.try_get("height")?;
                Ok(Some(height))
            },
            None => Ok(None)
        }
    }
}
