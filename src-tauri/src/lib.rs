use std::{net::SocketAddr, str::FromStr};

use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Manager, WebviewWindow, Window,
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};
use tokio::{
    io::AsyncWriteExt,
    sync::{Mutex, OnceCell},
};

use crate::{
    ngrok_wrapper::Header,
    proxy::{ProxyManager, ProxyMapping, proxy_impl::ProxyManagerImpl},
};

mod ngrok_wrapper;
mod proxy;

pub static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::const_new();
pub static PROXY_MANAGER: OnceCell<ProxyManagerImpl> = OnceCell::const_new();

pub async fn get_proxy_manager() -> &'static ProxyManagerImpl {
    PROXY_MANAGER
        .get_or_init(|| async { ProxyManagerImpl::new() })
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
        let _ = add_static_domain(&domain).await;
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
        None => get_token().await.map_err(|_e| format!("No token found"))?,
    };

    println!("Open session with {token}");

    ngrok_wrapper::get_session(token.clone()).await?;
    set_token(&token).await?;
    Ok(())
}

async fn get_token() -> Result<String, String> {
    let mut config_file = APP_HANDLE
        .get()
        .ok_or_else(|| format!("Cannot get app handle"))?
        .lock()
        .await
        .path()
        .app_config_dir()
        .map_err(|e| format!("Cannot get app config dir: {}", e))?;
    config_file.push("token.txt");

    let token = tokio::fs::read_to_string(config_file)
        .await
        .map_err(|e| format!("Cannot read file {e:?}"))?;

    if token.is_empty() {
        Err("Token file is empty".to_string())
    } else {
        Ok(token)
    }
}

async fn set_token(auth_token: &str) -> Result<(), String> {
    let mut config_file = APP_HANDLE
        .get()
        .ok_or_else(|| format!("Cannot get app handle"))?
        .lock()
        .await
        .path()
        .app_config_dir()
        .map_err(|e| format!("Cannot get app config dir: {}", e))?;

    tokio::fs::create_dir_all(&config_file).await.unwrap();
    config_file.push("token.txt");
    let mut file = tokio::fs::File::create(&config_file).await.unwrap();
    file.write_all(auth_token.as_bytes()).await.unwrap();

    Ok(())
}

async fn add_static_domain(domain: &str) -> Result<(), String> {
    let mut static_domains_file = APP_HANDLE
        .get()
        .unwrap()
        .lock()
        .await
        .path()
        .app_config_dir()
        .unwrap();

    tokio::fs::create_dir_all(&static_domains_file)
        .await
        .unwrap();
    static_domains_file.push("static_domains.txt");
    let mut file = tokio::fs::File::create(&static_domains_file).await.unwrap();
    file.write_all(domain.as_bytes()).await.unwrap();

    Ok(())
}

#[tauri::command]
async fn get_static_domains() -> Result<Vec<String>, String> {
    let mut static_domains_file = APP_HANDLE
        .get()
        .unwrap()
        .lock()
        .await
        .path()
        .app_config_dir()
        .unwrap();
    static_domains_file.push("static_domains.txt");

    let domains = tokio::fs::read_to_string(static_domains_file)
        .await
        .map_err(|e| format!("Cannot read file {e:?}"))?
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(domains)
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
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();

                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().is_ok_and(|v| v) {
                                hide_window_to_tray(app);
                            } else {
                                show_window_from_tray(app);
                            }
                        }
                    }
                })
                .build(app)?;

            APP_HANDLE.set(Mutex::new(app.handle().clone())).unwrap();

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let _ = window.hide_to_tray();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

trait HideToTray {
    fn hide_to_tray(&self);
    fn show_from_tray(&self);
}

impl HideToTray for WebviewWindow {
    fn hide_to_tray(&self) {
        let _ = self.hide();

        #[cfg(target_os = "macos")]
        {
            let _ = self
                .app_handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    }
    fn show_from_tray(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .app_handle()
                .set_activation_policy(tauri::ActivationPolicy::Regular);
        }

        let _ = self.unminimize();
        let _ = self.show();
        let _ = self.set_focus();
    }
}

impl HideToTray for Window {
    fn hide_to_tray(&self) {
        let _ = self.hide();

        #[cfg(target_os = "macos")]
        {
            let _ = self
                .app_handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    }
    fn show_from_tray(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .app_handle()
                .set_activation_policy(tauri::ActivationPolicy::Regular);
        }

        let _ = self.unminimize();
        let _ = self.show();
        let _ = self.set_focus();
    }
}

fn hide_window_to_tray(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.hide_to_tray();
    }
}

fn show_window_from_tray(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show_from_tray();
    }
}
