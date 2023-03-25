#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use env_logger::{Builder, Target};
use std::{
    fs::{self, OpenOptions},
    sync::{Arc, Mutex},
};
use tauri::{App, Manager, Window};
use tauri_runtime::GlobalShortcutManager;

mod logic;
mod utils;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, window: tauri::Window) -> String {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(5));
        window
            .emit(
                "inputPath",
                Payload {
                    message: "fake path".into(),
                },
            )
            .unwrap();
    });
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn handle_short_cut(card_win: Arc<Mutex<Window>>) {
    tauri::async_runtime::spawn(async move {
        let def_res = logic::get_def().await;
        if def_res.is_none() {
            info!("no word");
            return;
        }

        let def = def_res.unwrap();
        // println!("{}", def);
        card_win
            .lock()
            .unwrap()
            .emit("showDef", def)
            .unwrap_or_else(|e| println!("emmit error {}", e));
    });
}

fn init_log(app: &App) {
    let path_resolver = app.path_resolver();
    let log_path = path_resolver.app_log_dir().unwrap();
    if !log_path.exists() {
        fs::create_dir_all(&log_path).unwrap();
    }
    println!("log path {:?}", log_path);
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path.join("app.log"))
        .unwrap();
    Builder::from_default_env()
        .target(Target::Pipe(Box::new(log_file)))
        .init();
    info!("app start");
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
            init_log(app);
            let card_win = app.get_window("main").unwrap();
            let ref_win = Arc::new(Mutex::new(card_win));
            let mut gs_manager = app.global_shortcut_manager();
            gs_manager
                .register("Ctrl+Alt+c", move || handle_short_cut(ref_win.clone()))
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
