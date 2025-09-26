use async_trait::async_trait;
use std::net::SocketAddr;

pub mod http;
pub mod tcp;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Port already in use: {0}")]
    PortUnavailable(u16),
    #[error("Port not found: {0}")]
    PortNotFound(u16),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Address parse error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ProxyMapping {
    pub source_port: u16,
    pub target_addr: SocketAddr,
}

#[async_trait]
pub trait ProxyManager: Send + Sync {
    async fn add_mapping(&self, target_addr: SocketAddr) -> Result<ProxyMapping>;
    async fn remove_mapping(&self, source_port: u16) -> Result<()>;
    async fn list_mapping(&self) -> Vec<ProxyMapping>;
}
