use dashmap::DashMap;
use ngrok::{
    Session,
    config::{ForwarderBuilder, HttpTunnelBuilder},
    forwarder::Forwarder,
    tunnel::{EndpointInfo, HttpTunnel, TunnelCloser, TunnelInfo},
};
use tokio::sync::OnceCell;
use url::Url;

static TUNNELS: OnceCell<DashMap<String, Forwarder<HttpTunnel>>> = OnceCell::const_new();
static SESSION: OnceCell<Session> = OnceCell::const_new();

pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new_host_rewrite(host: impl Into<String>) -> Self {
        Header {
            name: "host".to_string(),
            value: host.into(),
        }
    }
}

pub async fn open_tunnel(
    domain: Option<impl Into<String>>,
    port: impl Into<String>,
    headers: Vec<Header>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = port.into();
    // Set up ngrok tunnel
    let session = get_session("").await.unwrap();

    let mut tunnel_builder: HttpTunnelBuilder = session.http_endpoint();
    if let Some(domain) = domain {
        tunnel_builder.domain(domain.into().clone());
    }

    for header in headers {
        tunnel_builder.request_header(header.name, header.value);
    }

    // Forward HTTP traffic from ngrok to the local server
    let tunnel: Forwarder<HttpTunnel> = tunnel_builder
        .listen_and_forward(Url::parse(&format!("http://localhost:{}", port.clone()))?)
        .await?;

    println!("Ngrok tunnel established at {}", tunnel.url());

    let tunnels = get_tunnels().await;
    tunnels.insert(tunnel.id().to_string(), tunnel);

    Ok(())
}

pub async fn close_tunnel(id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tunnels = get_tunnels().await;
    if let Some((_id, mut tunnel)) = tunnels.remove(id) {
        tunnel.close().await?;
        Ok(())
    } else {
        Err("Tunnel not found".into())
    }
}

pub async fn get_tunnels() -> &'static DashMap<String, Forwarder<HttpTunnel>> {
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
