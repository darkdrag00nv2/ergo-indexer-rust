use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, QueryBuilder, Row,
};

pub mod models;

#[derive(Debug, Clone)]
pub struct Database {
    pub db_conn: sqlx::Pool<sqlx::Postgres>,
}

impl Database {}
