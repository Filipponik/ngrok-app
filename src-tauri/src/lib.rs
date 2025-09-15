mod ngrok;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn create_tunnel(auth_token: &str, domain: &str, port: &str) -> Result<(), String> {
    let domain = if domain.is_empty() {
        None
    } else {
        Some(domain)
    };

    ngrok::create_tunnel(auth_token, domain, port)
        .await
        .unwrap();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, create_tunnel,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
