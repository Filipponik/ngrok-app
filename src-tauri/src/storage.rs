use async_trait::async_trait;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, thiserror::Error)]
pub struct StorageError<T: Display> {
    #[source]
    pub inner: T,
}

impl<T: Display> Display for StorageError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage error: {}", self.inner)
    }
}

impl<T: Display> StorageError<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
pub trait Storage {
    type ErrorType: Display;

    async fn save_static_domain(&self, domain: String)
    -> Result<(), StorageError<Self::ErrorType>>;

    async fn get_static_domains(&self) -> Result<HashSet<String>, StorageError<Self::ErrorType>>;

    async fn save_session_token(
        &self,
        &selftoken: String,
    ) -> Result<(), StorageError<Self::ErrorType>>;

    async fn get_session_token(&self) -> Result<String, StorageError<Self::ErrorType>>;
}
