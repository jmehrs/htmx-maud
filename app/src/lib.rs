use std::path::Path;

use include_dir::{Dir, include_dir};
use lazy_static::lazy_static;
use rusqlite_migration::{Migrations, Error as SQLITE_MIGRATION_ERROR};
use rusqlite::Error as SQLITE_ERROR;
use thiserror::Error;

pub mod device;
pub mod device_rest;
pub use rusqlite::Connection;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

#[derive(Error, Debug)]
pub enum DatabaseInitError {
    #[error("Error initializing SQLite3 DB")]
    InitError(#[from] SQLITE_ERROR),
    #[error("Error migrating SQLite3 DB")]
    MigrationError(#[from] SQLITE_MIGRATION_ERROR)
}

pub fn init_db<P>(path: P) -> Result<Connection, DatabaseInitError>
where
    P: AsRef<Path>
{
    let mut conn = Connection::open(path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

pub fn init_memory_db() -> Result<Connection, DatabaseInitError>{
    let mut conn = Connection::open_in_memory()?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

pub trait Command {
    fn execute(&self, conn: &Connection) -> Result<(), SQLITE_ERROR>;
}
