#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use tauri::{AppHandle, GlobalShortcutManager, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_log::LogTarget;
use utils::word;

mod logic;
mod utils;

#[tauri::command]
async fn lookup(word: String) -> Result<String, String> {
    match word::lookup(&word).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

fn handle_short_cut(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let def_res = logic::get_def().await;
        if def_res.is_none() {
            info!("no word");
            return;
        }

        let def = def_res.unwrap();
        lookup_window_visibility(&app);
        let win = app.get_window("lookup").unwrap();
        win.emit("showDef", def)
            .unwrap_or_else(|e| println!("emmit error {}", e));
    });
}

fn lookup_window_visibility(app: &AppHandle) {
    let item_handle = app.tray_handle().get_item("hide");
    let lookup_window = app.get_window("lookup").unwrap();
    if lookup_window.is_visible().unwrap() {
        lookup_window.hide().unwrap();
        item_handle.set_title("Show Lookup").unwrap();
    } else {
        lookup_window.show().unwrap();
        item_handle.set_title("Hide Lookup").unwrap();
    }
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                lookup_window_visibility(app);
            }
            _ => {}
        },
        SystemTrayEvent::DoubleClick { .. } => {
            lookup_window_visibility(app);
        }
        _ => {}
    }
}

fn build_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Lookup");
    let tray_menu = SystemTrayMenu::new().add_item(hide).add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .setup(move |app| {
            let app_handle = app.app_handle();
            let mut gs_manager = app.global_shortcut_manager();
            gs_manager
                .register("Ctrl+Shift+c", move || {
                    handle_short_cut(app_handle.clone());
                })
                .unwrap();
            Ok(())
        })
        .system_tray(build_tray())
        .on_system_tray_event(handle_tray_event)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let win = event.window();
                win.hide().unwrap();
                api.prevent_close();
                let item_handle = win.app_handle().tray_handle().get_item("hide");
                item_handle.set_title("Show Lookup").unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![lookup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
