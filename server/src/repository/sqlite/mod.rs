use crate::config::Cfg;
use crate::error::{Error, ErrorKind};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use super::Repository;

type DBPool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Clone)]
pub struct SqliteRepository {
    pool: DBPool,
}

impl SqliteRepository {
    pub fn conn(&self) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
        self.pool
            .get()
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))
    }
}

impl Repository for SqliteRepository {
    fn new(config: &Cfg) -> Result<Box<Self>, Error> {
        let manager = SqliteConnectionManager::file(config.database.filepath.clone());
        let pool = Pool::new(manager)
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        let repo = SqliteRepository { pool };
        repo.conn()?
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                password_hash TEXT NOT NULL
            )",
                (),
            )
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        repo.conn()?
            .execute(
                "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
                (),
            )
            .map_err(|err| Error::new(err.to_string(), ErrorKind::InternalError))?;

        Ok(Box::new(repo))
    }
}

pub mod setting;
pub mod user;
