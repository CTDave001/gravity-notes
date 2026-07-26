mod clipper;
mod commands;
mod export;
mod storage;

use clipper::clip_to_markdown;
use commands::*;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn create_capture_window(app: &AppHandle) {
    let window_label = format!(
        "capture-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );

    let url = WebviewUrl::App("index.html?window=capture".into());

    let builder = WebviewWindowBuilder::new(app, &window_label, url)
        .title("Quick Capture")
        .inner_size(500.0, 400.0)
        .min_inner_size(300.0, 200.0)
        .center()
        .resizable(true)
        .decorations(false)
        .visible(false);

    // Transparent windows work on Windows and macOS (with macos-private-api feature)
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let builder = builder.transparent(true).shadow(false);

    match builder.build() {
        Ok(window) => {
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
            let primary_modifier = if cfg!(target_os = "macos") {
                Modifiers::SUPER
            } else {
                Modifiers::CONTROL
            };
            let capture_shortcut =
                Shortcut::new(Some(primary_modifier | Modifiers::ALT), Code::KeyN);
            let focus_shortcut = Shortcut::new(Some(primary_modifier | Modifiers::ALT), Code::KeyG);

            let app_handle = app.handle().clone();
            if let Err(error) = app.global_shortcut().on_shortcut(
                capture_shortcut,
                move |_app, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        create_capture_window(&app_handle);
                    }
                },
            ) {
                log::warn!("Could not register the quick capture shortcut: {error}");
            }

            let app_handle = app.handle().clone();
            if let Err(error) =
                app.global_shortcut()
                    .on_shortcut(focus_shortcut, move |_app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            focus_main_window(&app_handle);
                        }
                    })
            {
                log::warn!("Could not register the focus shortcut: {error}");
            }

            #[cfg(target_os = "windows")]
            {
                let clip_shortcut =
                    Shortcut::new(Some(primary_modifier | Modifiers::ALT), Code::KeyV);
                if let Err(error) = app.global_shortcut().on_shortcut(
                    clip_shortcut,
                    move |_app, _shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            match clip_to_markdown() {
                                Ok(_) => log::info!("Clipboard converted to Markdown"),
                                Err(error) => {
                                    log::error!("Clip to markdown failed: {error}")
                                }
                            }
                        }
                    },
                ) {
                    log::warn!("Could not register the clipboard shortcut: {error}");
                }
            }

            // System tray
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let new_note = MenuItem::with_id(app, "new_note", "New Note", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Gravity", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show, &new_note, &quit])?;

            let mut tray_builder = TrayIconBuilder::new();
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            let _tray = tray_builder
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        let _ = app.emit("prepare-to-quit", ());
                        let app_handle = app.clone();
                        std::thread::spawn(move || {
                            std::thread::sleep(std::time::Duration::from_millis(750));
                            app_handle.exit(0);
                        });
                    }
                    "new_note" => {
                        create_capture_window(app);
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
            import_image,
            get_images_path,
            get_notes_path,
            search_notes,
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
                    if let Err(error) = window.hide() {
                        log::error!("Failed to hide the main window: {error}");
                    }
                    api.prevent_close();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {});
}
