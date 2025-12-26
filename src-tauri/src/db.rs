#![allow(dead_code, unused)]

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use redb::{Database, TableDefinition};
use tauri::Manager;
use tokio::sync::{Mutex, OnceCell};

use crate::{
    APP_HANDLE,
    storage::{Storage, StorageError},
};

// Table definitions
const CONFIG_TABLE: TableDefinition<&str, String> = TableDefinition::new("user_config");
const DOMAINS_TABLE: TableDefinition<&str, Vec<String>> = TableDefinition::new("user_domains");

// Database keys
const SESSION_TOKEN_KEY: &str = "session_token";
const DOMAINS_KEY: &str = "domains";

/// Database errors with more descriptive messages
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("App handle not initialized")]
    AppHandleNotInitialized,

    #[error("Failed to get app config directory: {0}")]
    ConfigDirectoryError(#[source] tauri::Error),

    #[error("Failed to create database directory: {0}")]
    DirectoryCreationError(#[source] std::io::Error),

    #[error("Database operation failed: {0}")]
    DatabaseError(#[from] redb::DatabaseError),

    #[error("Transaction failed: {0}")]
    TransactionError(#[from] redb::TransactionError),

    #[error("Commit failed: {0}")]
    CommitError(#[from] redb::CommitError),

    #[error("Storage operation failed: {0}")]
    StorageError(#[from] redb::StorageError),

    #[error("Table operation failed: {0}")]
    TableError(#[from] redb::TableError),

    #[error("Value not found")]
    ValueNotFound,
}

/// Result type alias for database operations
pub type DbResult<T> = Result<T, DbError>;

/// Shared database instance
static DATABASE: OnceCell<Arc<Mutex<Database>>> = OnceCell::const_new();

/// Initialize and get the shared database instance
async fn get_database() -> DbResult<Arc<Mutex<Database>>> {
    DATABASE
        .get_or_try_init(|| async {
            let db_path = get_database_path().await?;
            let database = Database::create(&db_path)?;
            Ok(Arc::new(Mutex::new(database)))
        })
        .await
        .cloned()
}

/// Get the database file path, creating directories if necessary
async fn get_database_path() -> DbResult<PathBuf> {
    let app_handle = APP_HANDLE
        .get()
        .ok_or(DbError::AppHandleNotInitialized)?
        .lock()
        .await;

    let mut config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(DbError::ConfigDirectoryError)?;

    // Create the config directory if it doesn't exist
    tokio::fs::create_dir_all(&config_dir)
        .await
        .map_err(DbError::DirectoryCreationError)?;

    config_dir.push("db.redb");
    Ok(config_dir)
}

/// Retrieve the session token from the database
async fn get_session_token() -> DbResult<String> {
    let database = get_database().await?;
    let db = database.lock().await;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(CONFIG_TABLE)?;

    table
        .get(SESSION_TOKEN_KEY)?
        .map(|value| value.value())
        .ok_or(DbError::ValueNotFound)
}

/// Save a session token to the database
async fn save_session_token(token: impl Into<String>) -> DbResult<()> {
    let database = get_database().await?;
    let db = database.lock().await;

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(CONFIG_TABLE)?;
        table.insert(SESSION_TOKEN_KEY, token.into())?;
    }
    write_txn.commit()?;

    Ok(())
}

/// Retrieve all static domains as a HashSet
async fn get_static_domains() -> DbResult<HashSet<String>> {
    let database = get_database().await?;
    let db = database.lock().await;

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(DOMAINS_TABLE)?;
    // daily-ample-kit.ngrok-free.app
    match table.get(DOMAINS_KEY)? {
        Some(domains) => Ok(HashSet::from_iter(domains.value())),
        None => Ok(HashSet::new()), // Return empty set if no domains exist
    }
}

/// Add a static domain to the database (idempotent operation)
async fn save_static_domain(domain: impl Into<String>) -> DbResult<()> {
    let domain = domain.into();
    let mut domains = get_static_domains().await?;

    // Early return if domain already exists
    if domains.contains(&domain) {
        return Ok(());
    }

    domains.insert(domain);
    save_all_static_domains(domains).await
}

/// Save all static domains to the database
async fn save_all_static_domains(domains: HashSet<String>) -> DbResult<()> {
    let database = get_database().await?;
    let db = database.lock().await;

    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(DOMAINS_TABLE)?;
        let domains_vec: Vec<String> = domains.into_iter().collect();
        table.insert(DOMAINS_KEY, domains_vec)?;
    }
    write_txn.commit()?;

    Ok(())
}

pub struct DbStorage;

#[async_trait]
impl Storage for DbStorage {
    type ErrorType = DbError;

    async fn save_static_domain(
        &self,
        domain: String,
    ) -> Result<(), StorageError<Self::ErrorType>> {
        save_static_domain(&domain).await.map_err(StorageError::new)
    }

    async fn get_static_domains(&self) -> Result<HashSet<String>, StorageError<Self::ErrorType>> {
        get_static_domains().await.map_err(StorageError::new)
    }

    async fn save_session_token(&self, token: String) -> Result<(), StorageError<Self::ErrorType>> {
        save_session_token(&token).await.map_err(StorageError::new)
    }

    async fn get_session_token(&self) -> Result<String, StorageError<Self::ErrorType>> {
        get_session_token().await.map_err(StorageError::new)
    }
}
