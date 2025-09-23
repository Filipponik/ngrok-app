use std::collections::HashSet;

use redb::{Database, TableDefinition};
use tauri::Manager;
use tokio::sync::OnceCell;

use crate::APP_HANDLE;

const CONFIG_TABLE: TableDefinition<&str, String> = TableDefinition::new("user_config");
const DOMAINS_TABLE: TableDefinition<&str, Vec<String>> = TableDefinition::new("user_domains");

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot take db file: {0}")]
    CannotTakeDbFile(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] redb::DatabaseError),
    #[error("No value")]
    NoValue,
    #[error("Transaction error: {0}")]
    TransactionError(#[from] redb::TransactionError),
    #[error("Commit error: {0}")]
    CommitError(#[from] redb::CommitError),
    #[error("Storage error: {0}")]
    StorageError(#[from] redb::StorageError),
    #[error("Table error: {0}")]
    TableError(#[from] redb::TableError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn get_session_token() -> Result<String> {
    let db = get_db().await?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(CONFIG_TABLE)?;

    Ok(table.get("session_token")?.ok_or(Error::NoValue)?.value())
}

pub async fn save_session_token(token: impl Into<String>) -> Result<()> {
    let db = get_db().await?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(CONFIG_TABLE)?;
        table.insert("session_token", token.into())?;
    }
    write_txn.commit()?;

    Ok(())
}

pub async fn get_static_domains() -> Result<HashSet<String>> {
    let db = get_db().await?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(DOMAINS_TABLE)?;

    Ok(HashSet::from_iter(
        table
            .get("domains")?
            .ok_or(Error::NoValue)?
            .value()
            .into_iter(),
    ))
}

pub async fn save_static_domain(domain: impl Into<String>) -> Result<()> {
    let mut domains = if let Ok(domains) = get_static_domains().await {
        domains
    } else {
        HashSet::new()
    };

    let domain = domain.into();
    if let Some(_existing_domain) = domains.get(&domain) {
        return Ok(());
    }

    domains.insert(domain);
    let db = get_db().await?;
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(DOMAINS_TABLE)?;
        table.insert("domains", domains.into_iter().collect::<Vec<String>>())?;
    }
    write_txn.commit()?;

    Ok(())
}

async fn get_db() -> Result<Database> {
    let db = Database::create(get_filename_cached().await?)?;
    Ok(db)
}

async fn get_filename() -> Result<String> {
    let mut static_domains_file = APP_HANDLE
        .get()
        .ok_or(Error::CannotTakeDbFile(
            "APP_HANDLE is not initialized".to_string(),
        ))?
        .lock()
        .await
        .path()
        .app_config_dir()
        .map_err(|err| {
            Error::CannotTakeDbFile(format!(
                "Cannot take app config directory: {}",
                err.to_string()
            ))
        })?;

    tokio::fs::create_dir_all(&static_domains_file)
        .await
        .map_err(|err| {
            Error::CannotTakeDbFile(format!("Cannot create directory: {}", err.to_string()))
        })?;

    static_domains_file.push("db.redb");

    Ok(static_domains_file.to_string_lossy().to_string())
}

static DATABASE_FILENAME: OnceCell<String> = OnceCell::const_new();

async fn get_filename_cached() -> Result<String> {
    DATABASE_FILENAME
        .get_or_try_init(|| async { get_filename().await })
        .await
        .cloned()
}
