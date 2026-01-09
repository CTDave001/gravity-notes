mod clipper;
mod commands;
mod export;
mod storage;

use clipper::clip_to_markdown;
use commands::*;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn create_capture_window(app: &AppHandle) {
    let window_label = format!("capture-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    let url = WebviewUrl::App("index.html?window=capture".into());

    match WebviewWindowBuilder::new(app, &window_label, url)
        .title("Quick Capture")
        .inner_size(500.0, 400.0)
        .min_inner_size(300.0, 200.0)
        .center()
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .visible(false)
        .build()
    {
        Ok(window) => {
            // Show and focus after a tiny delay to ensure it's ready
            let _ = window.show();
            let _ = window.set_focus();
        }
        Err(e) => {
            log::error!("Failed to create capture window: {}", e);
        }
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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
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
            let clip_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyV);

            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(capture_shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    create_capture_window(&app_handle);
                }
            })?;

            let app_handle = app.handle().clone();
            app.global_shortcut().on_shortcut(focus_shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    focus_main_window(&app_handle);
                }
            })?;

            // Web clipper: Ctrl+Alt+V - converts clipboard HTML to Markdown
            app.global_shortcut().on_shortcut(clip_shortcut, move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    match clip_to_markdown() {
                        Ok(_) => log::info!("Clipboard converted to Markdown"),
                        Err(e) => log::error!("Clip to markdown failed: {}", e),
                    }
                }
            })?;

            // System tray
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let new_note = MenuItem::with_id(app, "new_note", "New Note", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Gravity", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show, &new_note, &quit])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "new_note" => {
                        let _ = create_capture_window(app);
                    }
                    "show" => {
                        focus_main_window(app);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        focus_main_window(app);
                    }
                })
                .build(app)?;

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
            save_image,
            get_images_path,
            export::export_note_file,
            export::export_pdf,
            export::get_downloads_dir,
            export::reveal_in_folder,
            clip_to_markdown,
        ])
        .on_window_event(|window, event| {
            // Hide main window instead of closing it
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    window.hide().unwrap();
                    api.prevent_close();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Prevent app from exiting when all windows are closed
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
