use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, QueryBuilder, Row,
};
use anyhow::Result;

use crate::config::DatabaseSettings;

pub mod models;

#[derive(Debug, Clone)]
pub struct Database {
    pub db_conn: sqlx::Pool<sqlx::Postgres>,
}

impl Database {
    pub async fn new(database: &DatabaseSettings) -> Result<Self> {
        todo!()
    }
}
