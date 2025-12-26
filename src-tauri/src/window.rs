use tauri::{
    Manager, WebviewWindow, Window, WindowEvent,
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};

trait WindowLike {
    fn get_handle(&self) -> tauri::AppHandle;
    fn hide(&self) -> tauri::Result<()>;
    fn show(&self) -> tauri::Result<()>;
    fn unminimize(&self) -> tauri::Result<()>;
    fn set_focus(&self) -> tauri::Result<()>;
}

impl WindowLike for Window {
    fn get_handle(&self) -> tauri::AppHandle {
        self.app_handle().clone()
    }
    fn hide(&self) -> tauri::Result<()> {
        Window::hide(self)
    }
    fn show(&self) -> tauri::Result<()> {
        Window::show(self)
    }
    fn unminimize(&self) -> tauri::Result<()> {
        Window::unminimize(self)
    }
    fn set_focus(&self) -> tauri::Result<()> {
        Window::set_focus(self)
    }
}

impl WindowLike for WebviewWindow {
    fn get_handle(&self) -> tauri::AppHandle {
        self.app_handle().clone()
    }
    fn hide(&self) -> tauri::Result<()> {
        WebviewWindow::hide(self)
    }
    fn show(&self) -> tauri::Result<()> {
        WebviewWindow::show(self)
    }
    fn unminimize(&self) -> tauri::Result<()> {
        WebviewWindow::unminimize(self)
    }
    fn set_focus(&self) -> tauri::Result<()> {
        WebviewWindow::set_focus(self)
    }
}

pub trait HideToTray {
    fn hide_to_tray(&self);
    fn show_from_tray(&self);
}

impl<T: WindowLike> HideToTray for T {
    fn hide_to_tray(&self) {
        let _ = self.hide();

        #[cfg(target_os = "macos")]
        {
            let _ = self
                .get_handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory);
        }
    }

    fn show_from_tray(&self) {
        #[cfg(target_os = "macos")]
        {
            let _ = self
                .get_handle()
                .set_activation_policy(tauri::ActivationPolicy::Regular);
        }

        let _ = self.unminimize();
        let _ = self.show();
        let _ = self.set_focus();
    }
}

pub fn window_event_handler(window: &Window, event: &WindowEvent) {
    if let WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        window.hide_to_tray();
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
