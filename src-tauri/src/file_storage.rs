use crate::storage::{Storage, StorageError};
use async_trait::async_trait;
use std::{collections::HashSet, path::PathBuf};
use tauri::{AppHandle, Manager};
use tokio::io::AsyncWriteExt;

#[derive(Debug, thiserror::Error)]
pub enum FileStorageError {
    #[error("Failed to create directory")]
    DirectoryCreationError(std::io::Error),
    #[error("Config directory error")]
    ConfigDirectoryError(tauri::Error),
    #[error("Cannot read file {0:?}")]
    ReadFileError(std::io::Error),
    #[error("Cannot write file {0:?}")]
    WriteFileError(std::io::Error),
    #[error("Cannot create file {0:?}")]
    CreateFileError(std::io::Error),
    #[error("Token file is empty")]
    EmptyTokenFile,
}

#[derive(Debug)]
pub struct FileStorage {
    token_file_path: PathBuf,
    domains_file_path: PathBuf,
}

impl FileStorage {
    pub fn new(app_handle: &AppHandle) -> Result<Self, FileStorageError> {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .map_err(FileStorageError::ConfigDirectoryError)?;

        // Create the config directory if it doesn't exist
        std::fs::create_dir_all(&config_dir).map_err(FileStorageError::DirectoryCreationError)?;
        let r = Self {
            token_file_path: config_dir.join("token.txt"),
            domains_file_path: config_dir.join("domains.txt"),
        };
        dbg!(&r);
        Ok(r)
    }
}

impl FileStorage {
    async fn read_string(path: &PathBuf) -> Result<String, StorageError<FileStorageError>> {
        tokio::fs::read_to_string(path)
            .await
            .map_err(|err| StorageError::new(FileStorageError::ReadFileError(err)))
    }

    async fn rewrite_file(
        path: &PathBuf,
        content: &str,
    ) -> Result<(), StorageError<FileStorageError>> {
        let mut file = tokio::fs::File::create(path)
            .await
            .map_err(|err| StorageError::new(FileStorageError::CreateFileError(err)))?;

        file.write_all(content.as_bytes())
            .await
            .map_err(|err| StorageError::new(FileStorageError::WriteFileError(err)))
    }
}

#[async_trait]
impl Storage for FileStorage {
    type ErrorType = FileStorageError;

    async fn save_static_domain(
        &self,
        domain: String,
    ) -> Result<(), StorageError<Self::ErrorType>> {
        let mut existing_domains = self.get_static_domains().await?;
        existing_domains.insert(domain);
        let file_content = existing_domains
            .iter()
            .fold(String::new(), |acc, domain| acc + domain + "\n");

        Self::rewrite_file(&self.domains_file_path, &file_content).await?;
        Ok(())
    }

    async fn get_static_domains(&self) -> Result<HashSet<String>, StorageError<Self::ErrorType>> {
        Ok(HashSet::from_iter(
            Self::read_string(&self.domains_file_path)
                .await?
                .split('\n')
                .filter(|domain| !domain.is_empty())
                .map(|domain| domain.to_string()),
        ))
    }

    async fn save_session_token(&self, token: String) -> Result<(), StorageError<Self::ErrorType>> {
        Self::rewrite_file(&self.token_file_path, &token).await
    }

    async fn get_session_token(&self) -> Result<String, StorageError<Self::ErrorType>> {
        let token = Self::read_string(&self.token_file_path).await?;

        if token.is_empty() {
            Err(StorageError::new(FileStorageError::EmptyTokenFile))
        } else {
            Ok(token)
        }
    }
}
