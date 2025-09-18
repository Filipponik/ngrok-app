use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Manager, WebviewWindow, Window,
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};
use tokio::{
    io::AsyncWriteExt,
    sync::{Mutex, OnceCell},
};

use crate::ngrok_wrapper::Header;

mod ngrok_wrapper;

pub static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::const_new();

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

    ngrok_wrapper::open_tunnel(domain.clone(), command.port, headers, command.basic_auth)
        .await
        .map_err(|e| format!("Failed to create tunnel: {}", e))?;

    if let Some(domain) = domain {
        let _ = add_static_domain(&domain).await;
    }

    Ok(())
}

#[tauri::command]
async fn tunnel_close(id: &str) -> Result<(), String> {
    ngrok_wrapper::close_tunnel(id)
        .await
        .expect("Failed to create tunnel");

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
struct TunnelResponse {
    id: String,
    url: String,
    port: String,
    is_static_domain: bool,
    headers: Vec<Header>,
    basic_auth: Option<BasicAuth>,
}

#[tauri::command]
async fn tunnel_list() -> Vec<TunnelResponse> {
    ngrok_wrapper::get_tunnels()
        .await
        .into_iter()
        .map(|tunnel| {
            let tunnel = tunnel.value();
            TunnelResponse {
                id: tunnel.id.clone(),
                url: tunnel.url.clone(),
                port: tunnel.port.clone(),
                is_static_domain: tunnel.is_static_domain,
                headers: tunnel.request_headers.clone(),
                basic_auth: tunnel.basic_auth.clone(),
            }
        })
        .collect()
}

#[tauri::command]
async fn open_session(auth_token: Option<&str>) -> Result<(), String> {
    let token = if let Some(input_token) = auth_token {
        input_token.to_string()
    } else {
        if let Ok(token) = get_token().await {
            token
        } else {
            return Err("No token found".to_string());
        }
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
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide_to_tray();
            }
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
