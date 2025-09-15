use dashmap::DashMap;
use ngrok::{
    Session,
    config::{ForwarderBuilder, HttpTunnelBuilder},
    forwarder::Forwarder,
    tunnel::{EndpointInfo, HttpTunnel, TunnelInfo},
};
use tokio::sync::OnceCell;
use url::Url;

static TUNNELS: OnceCell<DashMap<String, Forwarder<HttpTunnel>>> = OnceCell::const_new();
static SESSION: OnceCell<Session> = OnceCell::const_new();

pub async fn create_tunnel(
    auth_token: impl Into<String>,
    domain: Option<impl Into<String>>,
    port: impl Into<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = port.into();
    // Set up ngrok tunnel
    let session = get_session(auth_token).await;

    let mut tunnel_builder: HttpTunnelBuilder = session.http_endpoint();
    if let Some(domain) = domain {
        tunnel_builder.domain(domain.into().clone());
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

async fn get_tunnels() -> &'static DashMap<String, Forwarder<HttpTunnel>> {
    TUNNELS.get_or_init(|| async { DashMap::new() }).await
}

async fn get_session(auth_token: impl Into<String>) -> &'static Session {
    SESSION
        .get_or_init(|| async {
            ngrok::Session::builder()
                .authtoken(auth_token)
                .connect()
                .await
                .unwrap()
        })
        .await
}
