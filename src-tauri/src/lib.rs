use std::{net::SocketAddr, str::FromStr};

use crate::{
    ngrok_wrapper::Header,
    proxy::{ProxyManager, ProxyMapping, tcp::TcpProxyManager},
};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tokio::sync::{Mutex, OnceCell};

mod db;
mod ngrok_wrapper;
mod proxy;
mod window;

pub static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::const_new();
pub static PROXY_MANAGER: OnceCell<TcpProxyManager> = OnceCell::const_new();

pub async fn get_proxy_manager() -> &'static TcpProxyManager {
    PROXY_MANAGER
        .get_or_init(|| async { TcpProxyManager::new() })
        .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct BasicAuth {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TunnelOpen {
    port: String,
    domain: Option<String>,
    host_rewrite: Option<String>,
    headers: Vec<Header>,
    basic_auth: Option<BasicAuth>,
}

#[tauri::command]
async fn tunnel_open(command: TunnelOpen) -> Result<(), String> {
    let domain = command
        .domain
        .and_then(|d| if d.is_empty() { None } else { Some(d) });
    let host_rewrite = command
        .host_rewrite
        .clone()
        .and_then(|h| if h.is_empty() { None } else { Some(h) });

    let mut headers = command.headers;
    if let Some(host) = host_rewrite {
        headers.push(Header::new_host_rewrite(host));
    }

    let mapping: ProxyMapping = get_proxy_manager()
        .await
        .add_mapping(SocketAddr::from_str(&format!("0.0.0.0:{}", command.port)).unwrap())
        .await
        .map_err(|e| format!("Failed to add mapping: {}", e))?;
    let proxy_port = mapping.source_port;

    ngrok_wrapper::open_tunnel(
        domain.clone(),
        proxy_port.to_string(),
        headers,
        command.basic_auth,
    )
    .await
    .map_err(|e| format!("Failed to create tunnel: {}", e))?;

    if let Some(domain) = domain {
        let _ = db::save_static_domain(&domain).await;
    }

    Ok(())
}

#[tauri::command]
async fn tunnel_close(id: &str) -> Result<(), String> {
    let tunnel_port: u16 = ngrok_wrapper::get_tunnels()
        .await
        .iter()
        .find(|tunnel| tunnel.value().id == id)
        .ok_or(format!("Failed to find tunnel: {}", id))?
        .port
        .parse()
        .map_err(|e| format!("Failed to parse port: {}", e))?;

    ngrok_wrapper::close_tunnel(id)
        .await
        .map_err(|e| format!("Failed to close tunnel: {}", e))?;

    get_proxy_manager()
        .await
        .remove_mapping(tunnel_port)
        .await
        .map_err(|e| format!("Failed to close tunnel: {}", e))?;

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
struct TunnelResponse {
    id: String,
    url: String,
    local_port: u16,
    proxy_port: u16,
    is_static_domain: bool,
    request_headers: Vec<Header>,
    basic_auth: Option<BasicAuth>,
}

#[tauri::command]
async fn tunnel_list() -> Vec<TunnelResponse> {
    let mappings = get_proxy_manager().await.list_mapping().await;

    ngrok_wrapper::get_tunnels()
        .await
        .into_iter()
        .map(|tunnel| {
            let proxy_port: u16 = tunnel.port.clone().parse().unwrap();
            let tunnel = tunnel.value();
            let local_port = mappings
                .iter()
                .find(|mapping| mapping.source_port == proxy_port)
                .unwrap()
                .target_addr
                .port();

            TunnelResponse {
                id: tunnel.id.clone(),
                url: tunnel.url.clone(),
                local_port: local_port,
                proxy_port: proxy_port,
                is_static_domain: tunnel.is_static_domain,
                request_headers: tunnel.request_headers.clone(),
                basic_auth: tunnel.basic_auth.clone(),
            }
        })
        .collect()
}

#[tauri::command]
async fn open_session(auth_token: Option<&str>) -> Result<(), String> {
    let token = match auth_token {
        Some(input_token) => input_token.to_string(),
        None => db::get_session_token()
            .await
            .map_err(|_e| format!("No token found"))?,
    };

    println!("Open session with {token}");

    ngrok_wrapper::get_session(token.clone()).await?;
    db::save_session_token(&token)
        .await
        .map_err(|err| format!("Failed to save session token: {}", err))?;
    Ok(())
}

#[tauri::command]
async fn get_static_domains() -> Result<Vec<String>, String> {
    Ok(db::get_static_domains()
        .await
        .map_err(|err| err.to_string())?
        .into_iter()
        .collect())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_prevent_default::init())
        .invoke_handler(tauri::generate_handler![
            tunnel_open,
            tunnel_close,
            open_session,
            tunnel_list,
            get_static_domains,
        ])
        .setup(|app: &mut tauri::App| {
            window::window_setup_handler(app)?;
            APP_HANDLE.set(Mutex::new(app.handle().clone())).unwrap();

            Ok(())
        })
        .on_window_event(window::window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
