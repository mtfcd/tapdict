#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{Manager, Window};
use tauri_runtime::GlobalShortcutManager;

mod ocr;
mod os_utils;
mod word;

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

async fn show_def(word: String, card_win: Arc<Mutex<Window>>) {
    let def_res = word::lookup(&word).await;
    if let Err(e) = &def_res {
        println!("lookup error: {}", e);
        return;
    }

    let def = def_res.unwrap();
    println!("{}", def);
    card_win
        .lock()
        .unwrap()
        .emit("showDef", def)
        .unwrap_or_else(|e| println!("emmit error {}", e));
}

fn handle_short_cut(card_win: Arc<Mutex<Window>>) {
    let start = Instant::now();
    let (img, pos) = os_utils::get_mouse_position();
    let word_res = ocr::extract_word(img.buffer().to_vec(), pos);
    if let Err(e) = word_res {
        println!("get word error: {}", e.to_string());
        return;
    }

    let word = word_res.unwrap();
    println!("get word take: {:?} {}", start.elapsed(), &word);
    tauri::async_runtime::spawn(async move {
        show_def(word, card_win).await;
        println!("oce take: {:?}", start.elapsed());
    });
}

fn main() {
    tauri::Builder::default()
        .setup(move |app| {
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
