use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::{
    io::AsyncWriteExt,
    sync::{Mutex, OnceCell},
};

use crate::ngrok_wrapper::Header;

mod ngrok_wrapper;

pub static APP_HANDLE: OnceCell<Mutex<AppHandle>> = OnceCell::const_new();

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TunnelOpen {
    port: String,
    domain: Option<String>,
    host_rewrite: Option<String>,
    headers: Vec<Header>,
}

#[tauri::command]
async fn tunnel_open(command: TunnelOpen) -> Result<(), String> {
    let domain = command
        .domain
        .clone()
        .and_then(|d| if d.is_empty() { None } else { Some(d) });
    let host_rewrite = command
        .host_rewrite
        .clone()
        .and_then(|h| if h.is_empty() { None } else { Some(h) });

    let mut headers = command.headers;
    if let Some(host) = host_rewrite {
        headers.push(Header::new_host_rewrite(host));
    }

    ngrok_wrapper::open_tunnel(domain, command.port, headers)
        .await
        .expect("Failed to create tunnel");

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
    set_token(&token).await;
    Ok(())
}

async fn get_token() -> Result<String, String> {
    let mut config_file = APP_HANDLE
        .get()
        .unwrap()
        .lock()
        .await
        .path()
        .app_config_dir()
        .unwrap();
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

async fn set_token(auth_token: &str) {
    let mut config_file = APP_HANDLE
        .get()
        .unwrap()
        .lock()
        .await
        .path()
        .app_config_dir()
        .unwrap();

    tokio::fs::create_dir_all(&config_file).await.unwrap();
    config_file.push("token.txt");
    let mut file = tokio::fs::File::create(&config_file).await.unwrap();
    file.write_all(auth_token.as_bytes()).await.unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            tunnel_open,
            tunnel_close,
            open_session,
            tunnel_list,
        ])
        .setup(|app: &mut tauri::App| {
            APP_HANDLE.set(Mutex::new(app.handle().clone())).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
