#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, GlobalShortcutManager, Manager, Window};
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

fn handle_short_cut(card_win: Arc<Mutex<Window>>) {
    tauri::async_runtime::spawn(async move {
        let def_res = logic::get_def().await;
        if def_res.is_none() {
            info!("no word");
            return;
        }

        let def = def_res.unwrap();
        let win = card_win.lock().unwrap();
        if !win.is_visible().unwrap() {
            win.show().unwrap();
        }
        win.emit("showDef", def)
            .unwrap_or_else(|e| println!("emmit error {}", e));
    });
}

fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                app.get_window("lookup").unwrap().hide().unwrap();
            }
            _ => {}
        },
        SystemTrayEvent::DoubleClick { .. } => {
            app.get_window("lookup").unwrap().show().unwrap();
        }
        _ => {}
    }
}

fn build_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Window");
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
            let card_win = app.get_window("lookup").unwrap();
            let ref_win = Arc::new(Mutex::new(card_win));
            let mut gs_manager = app.global_shortcut_manager();
            gs_manager
                .register("Ctrl+Shift+c", move || handle_short_cut(ref_win.clone()))
                .unwrap();
            Ok(())
        })
        .system_tray(build_tray())
        .on_system_tray_event(handle_tray_event)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![lookup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
