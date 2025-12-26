use async_trait::async_trait;
use dashmap::DashMap;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, body::Incoming};
use hyper_util::rt::TokioIo;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};

use crate::proxy::{Error, ProxyManager, ProxyMapping, Result};

#[derive(Debug, Clone)]
pub struct HttpProxyManager {
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

impl Default for HttpProxyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProxyManager {
    pub fn new() -> Self {
        let (control_tx, _) = broadcast::channel(100);
        Self {
            mappings: Arc::new(DashMap::new()),
            control_tx,
        }
    }

    async fn start_proxy_listener_on_random_port(&self, target_addr: SocketAddr) -> Result<u16> {
        let (listener, port) = self.bind_random_port().await?;
        let control_tx = self.control_tx.clone();

        tokio::spawn(async move {
            let mut shutdown_rx = control_tx.subscribe();

            tokio::select! {
                result = Self::proxy_loop(listener, target_addr) => {
                    if let Err(e) = result {
                        eprintln!("HTTP proxy error: {}", e);
                    }
                },
                _ = async {
                    while let Ok(ControlMessage::StopProxy(p)) = shutdown_rx.recv().await {
                        if port == p {
                            break;
                        }
                    }
                } => {
                    println!("HTTP proxy listener stopped");
                }
            }
        });

        Ok(port)
    }

    async fn proxy_loop(listener: TcpListener, target_addr: SocketAddr) -> Result<()> {
        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let io = TokioIo::new(stream);

                let service = service_fn(move |req| Self::handle_request(req, target_addr));

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("HTTP connection error: {}", err);
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

    async fn handle_request(
        req: Request<Incoming>,
        target_addr: SocketAddr,
    ) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
        let uri = req.uri().clone();
        let method = req.method().clone();

        let target_uri = format!(
            "http://{}{}",
            target_addr,
            uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/")
        );

        println!("HTTP proxy request: {} {}", method, target_uri);

        match Self::forward_request(req, &target_uri).await {
            Ok(response) => Ok(response),
            Err(e) => {
                eprintln!("Failed to forward request {} {}: {}", method, uri, e);
                Ok(Response::builder()
                    .status(StatusCode::BAD_GATEWAY)
                    .body(Full::new(Bytes::from(format!("Proxy error: {}", e))))
                    .unwrap())
            }
        }
    }

    async fn forward_request(
        req: Request<Incoming>,
        target_uri: &str,
    ) -> std::result::Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        let method = req.method().clone();
        let headers = req.headers().clone();
        let body_bytes = req.into_body().collect().await?.to_bytes();

        let mut new_req = Request::builder().method(method).uri(target_uri);

        for (name, value) in headers.iter() {
            if name != "host" {
                new_req = new_req.header(name, value);
            }
        }

        let new_req = new_req.body(Full::new(body_bytes))?;
        let response = client.request(new_req).await?;

        let status = response.status();
        let response_headers = response.headers().clone();
        let response_body = response.into_body().collect().await?.to_bytes();

        println!("HTTP proxy response: {}", status);

        let mut response_builder = Response::builder().status(status);

        for (name, value) in response_headers.iter() {
            response_builder = response_builder.header(name, value);
        }

        Ok(response_builder.body(Full::new(response_body))?)
    }
}

#[async_trait]
impl ProxyManager for HttpProxyManager {
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
