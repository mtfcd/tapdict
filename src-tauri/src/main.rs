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

mod logic;
mod utils;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
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
                app.get_window("main").unwrap().hide().unwrap();
            }
            _ => {}
        },
        SystemTrayEvent::DoubleClick { .. } => {
            app.get_window("main").unwrap().show().unwrap();
        }
        _ => {}
    }
}

fn build_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let reindex = CustomMenuItem::new("reindex".to_string(), "Reindex");
    let upgrade = CustomMenuItem::new("upgrade".to_string(), "Upgrade");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide Window");
    let tray_menu = SystemTrayMenu::new()
        .add_item(upgrade)
        // .add_item(reindex)
        .add_item(hide)
        .add_item(quit);
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
            let card_win = app.get_window("main").unwrap();
            let ref_win = Arc::new(Mutex::new(card_win));
            let mut gs_manager = app.global_shortcut_manager();
            gs_manager
                .register("Ctrl+Alt+c", move || handle_short_cut(ref_win.clone()))
                .unwrap();
            Ok(())
        })
        .system_tray(build_tray())
        .on_system_tray_event(|app, event| handle_tray_event(app, event))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
