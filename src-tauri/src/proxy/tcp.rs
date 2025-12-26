#![allow(dead_code, unused)]

use async_trait::async_trait;
use dashmap::DashMap;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};

use crate::proxy::{Error, ProxyManager, ProxyMapping, Result};

#[derive(Debug, Clone)]
pub struct TcpProxyManager {
    mappings: Arc<DashMap<u16, ProxyMappingInternal>>,
    control_tx: broadcast::Sender<ControlMessage>,
}

#[derive(Debug, Clone)]
enum ControlMessage {
    StopProxy(u16),
}

#[derive(Debug)]
struct ProxyMappingInternal {
    target_addr: SocketAddr,
}

impl TcpProxyManager {
    pub fn new() -> Self {
        let (control_tx, _) = broadcast::channel(100);
        Self {
            mappings: Arc::new(DashMap::new()),
            control_tx,
        }
    }

    async fn start_proxy_listener_on_random_port(&self, target_addr: SocketAddr) -> Result<u16> {
        let (listener, port) = self.bind_random_port().await?;

        // Clone only what we need BEFORE creating the task
        let control_tx = self.control_tx.clone();

        tokio::spawn(async move {
            let mut shutdown_rx = control_tx.subscribe();

            tokio::select! {
                result = Self::proxy_loop(listener, target_addr, control_tx) => {
                    if let Err(e) = result {
                        eprintln!("Proxy error: {}", e);
                    }
                },
                _ = async {
                    while let Ok(ControlMessage::StopProxy(p)) = shutdown_rx.recv().await {
                        if port == p {
                            break;
                        }
                    }
                } => {
                    println!("Proxy listener stopped");
                }
            }
        });

        Ok(port)
    }

    async fn proxy_loop(
        listener: TcpListener,
        target_addr: SocketAddr,
        control_tx: broadcast::Sender<ControlMessage>,
    ) -> Result<()> {
        let port: u16 = listener.local_addr()?.port();
        while let Ok((inbound, _)) = listener.accept().await {
            let control_tx = control_tx.clone();
            tokio::spawn(async move {
                println!("Proxy started");
                if let Err(e) =
                    Self::handle_proxy_connection(port, inbound, target_addr, control_tx).await
                {
                    eprintln!("Proxy error: {}", e);
                }
            });
        }
        Ok(())
    }

    async fn bind_random_port(&self) -> Result<(TcpListener, u16)> {
        let listener = TcpListener::bind("0.0.0.0:0")
            .await
            .map_err(|_| Error::PortUnavailable(0))?;

        let port = listener
            .local_addr()
            .map_err(|_| Error::PortUnavailable(0))?
            .port();

        Ok((listener, port))
    }

    async fn handle_proxy_connection(
        port: u16,
        inbound: tokio::net::TcpStream,
        target_addr: SocketAddr,
        control_tx: broadcast::Sender<ControlMessage>,
    ) -> Result<()> {
        let mut inbound = inbound;
        let mut outbound = tokio::net::TcpStream::connect(target_addr).await?;
        let mut shutdown_rx = control_tx.subscribe();

        tokio::select! {
            result = tokio::io::copy_bidirectional(&mut inbound, &mut outbound) => {
                result?;
            },
            _ = async {
                while let Ok(ControlMessage::StopProxy(p)) = shutdown_rx.recv().await {
                    if port == p {
                        break;
                    }
                }
            } => {
                println!("Proxy connection stopped");
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ProxyManager for TcpProxyManager {
    async fn add_mapping(&self, target_addr: SocketAddr) -> Result<ProxyMapping> {
        let port = self
            .start_proxy_listener_on_random_port(target_addr)
            .await?;

        let mapping_internal = ProxyMappingInternal { target_addr };

        self.mappings.insert(port, mapping_internal);

        Ok(ProxyMapping {
            source_port: port,
            target_addr,
        })
    }

    async fn remove_mapping(&self, source_port: u16) -> Result<()> {
        self.control_tx
            .send(ControlMessage::StopProxy(source_port))
            .map_err(|_| Error::PortNotFound(source_port))?;

        // Remove from map after sending signal
        self.mappings.remove(&source_port);
        Ok(())
    }

    async fn list_mapping(&self) -> Vec<ProxyMapping> {
        self.mappings
            .iter()
            .map(|item| ProxyMapping {
                source_port: *item.key(),
                target_addr: item.value().target_addr,
            })
            .collect()
    }
}
