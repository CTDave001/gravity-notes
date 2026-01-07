mod commands;
mod storage;

use commands::*;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

fn create_capture_window(app: &AppHandle) {
    let window_label = format!("capture-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    let url = WebviewUrl::App("index.html?window=capture".into());

    if let Err(e) = WebviewWindowBuilder::new(app, &window_label, url)
        .title("Quick Capture")
        .inner_size(500.0, 400.0)
        .center()
        .resizable(true)
        .always_on_top(true)
        .build()
    {
        log::error!("Failed to create capture window: {}", e);
    }
}

fn focus_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Register global shortcuts
            let capture_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN);
            let focus_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyG);

            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(capture_shortcut, move |_app, _shortcut, _event| {
                create_capture_window(&app_handle);
            })?;

            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(focus_shortcut, move |_app, _shortcut, _event| {
                focus_main_window(&app_handle);
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_note,
            save_note,
            delete_note,
            get_note,
            list_notes,
            delete_if_empty,
            cleanup_empty_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
