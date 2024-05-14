use sqlx::{Pool, Sqlite, SqlitePool};

use crate::error::Error;
use std::fs::File;

pub const DBPATH: &str = "./data/users.db";

pub struct Dao {
    pool: sqlx::SqlitePool,
}

impl Dao {
    pub async fn new() -> Result<Self, Error> {
        File::create(DBPATH).or_else(Error::database_initialization_error)?;
        let pool: Pool<Sqlite> = SqlitePool::connect(DBPATH)
            .await
            .or_else(Error::database_connection_failed)?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &sqlx::SqlitePool {
        &self.pool
    }
}
