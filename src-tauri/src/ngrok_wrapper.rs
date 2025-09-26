use dashmap::DashMap;
use ngrok::{
    Session,
    config::{ForwarderBuilder, HttpTunnelBuilder},
    forwarder::Forwarder,
    tunnel::{EndpointInfo, HttpTunnel, TunnelCloser, TunnelInfo},
};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use url::Url;

use crate::BasicAuth;

static TUNNELS: OnceCell<DashMap<String, TunnelOpened>> = OnceCell::const_new();
static SESSION: OnceCell<Session> = OnceCell::const_new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new_host_rewrite(host: impl Into<String>) -> Self {
        Header {
            name: "Host".to_string(),
            value: host.into(),
        }
    }
}

#[derive(Serialize)]
pub struct TunnelOpened {
    pub id: String,
    pub url: String,
    pub port: String,
    pub is_static_domain: bool,
    pub request_headers: Vec<Header>,
    pub response_headers: Vec<Header>,
    pub basic_auth: Option<BasicAuth>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(skip)]
    inner: Forwarder<HttpTunnel>,
}

pub async fn open_tunnel(
    domain: Option<impl Into<String>>,
    port: impl Into<String>,
    request_headers: Vec<Header>,
    response_headers: Vec<Header>,
    basic_auth: Option<BasicAuth>,
    name: Option<String>,
    description: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = port.into();
    let domain: Option<String> = domain.and_then(|domain| Some(domain.into()));
    // Set up ngrok tunnel
    let session = get_session("").await.unwrap();

    let mut tunnel_builder: HttpTunnelBuilder = session.http_endpoint();
    if let Some(domain) = domain.clone() {
        tunnel_builder.domain(domain);
    }

    for header in &request_headers {
        tunnel_builder.request_header(header.name.clone(), header.value.clone());
    }

    for header in &response_headers {
        tunnel_builder.response_header(header.name.clone(), header.value.clone());
    }

    if let Some(basic_auth) = basic_auth.clone() {
        tunnel_builder.basic_auth(basic_auth.username.clone(), basic_auth.password.clone());
    }

    // Forward HTTP traffic from ngrok to the local server
    let tunnel: Forwarder<HttpTunnel> = tunnel_builder
        .listen_and_forward(Url::parse(&format!("http://localhost:{}", port.clone()))?)
        .await?;

    let tunnel_name = name.as_deref().unwrap_or("Unnamed");
    println!(
        "Ngrok tunnel '{}' established at {}",
        tunnel_name,
        tunnel.url()
    );

    let tunnels = get_tunnels().await;
    let tunnel_opened = TunnelOpened {
        id: tunnel.id().to_string(),
        url: tunnel.url().to_string(),
        port,
        is_static_domain: domain.is_some(),
        request_headers: request_headers,
        response_headers: response_headers,
        basic_auth: basic_auth,
        name: name,
        description: description,
        inner: tunnel,
    };

    tunnels.insert(tunnel_opened.id.clone(), tunnel_opened);

    Ok(())
}

pub async fn close_tunnel(id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tunnels = get_tunnels().await;
    if let Some((_id, mut tunnel)) = tunnels.remove(id) {
        tunnel.inner.close().await?;
        Ok(())
    } else {
        Err("Tunnel not found".into())
    }
}

pub async fn get_tunnels() -> &'static DashMap<String, TunnelOpened> {
    TUNNELS.get_or_init(|| async { DashMap::new() }).await
}

pub async fn get_session(auth_token: impl Into<String>) -> Result<&'static Session, String> {
    SESSION
        .get_or_try_init(|| async {
            ngrok::Session::builder()
                .authtoken(auth_token)
                .connect()
                .await
        })
        .await
        .map_err(|e| format!("Failed to create ngrok session: {}", e))
}
