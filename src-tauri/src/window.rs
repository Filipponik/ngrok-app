use tauri::{
    Manager, WebviewWindow, Window, WindowEvent,
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};

pub trait HideToTray {
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

pub fn window_event_handler(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide_to_tray();
        }
        _ => {}
    }
}

pub fn window_setup_handler(app: &mut tauri::App) -> Result<(), tauri::Error> {
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
                        window.hide_to_tray();
                    } else {
                        window.show_from_tray();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
