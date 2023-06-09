#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate sqlx;

use sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, SqliteConnection};
use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, State,
    SystemTray, SystemTrayEvent, SystemTrayMenu,
};
use tauri_plugin_log::LogTarget;

mod logic;
mod utils;
use utils::dict;

struct Db {
    connection: Mutex<Option<SqliteConnection>>,
}

#[tauri::command]
async fn lookup(word: String, db: State<'_, Db>) -> Result<String, String> {
    let mut g = db.connection.lock().await;
    let conn = g.as_mut();
    match dict::lookup(&word, conn).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn lookup_prs(word: String) -> Result<String, String> {
    match dict::lookup_pronounce_from_mw(&word).await {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn close_window(app: AppHandle) {
    hide_window(&app)
}

fn handle_short_cut(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let word = logic::get_word().await;
        if word.is_none() {
            info!("no word");
            return;
        }
        let word = word.unwrap();
        info!("get word: {}", &word);

        let db = app.state::<Db>();
        let def = match dict::lookup(&word, db.connection.lock().await.as_mut()).await {
            Ok(res) => res,
            Err(e) => {
                error!("get def: {}", e);
                return;
            }
        };
        show_window(&app);
        app.get_window("lookup")
            .unwrap()
            .emit("showDef", def)
            .unwrap_or_else(|e| println!("emmit error {}", e));
    });
}

fn hide_window(app: &AppHandle) {
    app.get_window("lookup").unwrap().hide().unwrap();
    app.tray_handle()
        .get_item("hide")
        .set_title("Show Lookup")
        .unwrap();
}

fn show_window(app: &AppHandle) {
    let lookup_window = app.get_window("lookup").unwrap();
    lookup_window.show().unwrap();
    let item_handle = app.tray_handle().get_item("hide");
    item_handle.set_title("Hide Lookup").unwrap();
}

fn toggle_window_visibility(app: &AppHandle) {
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
                toggle_window_visibility(app);
            }
            _ => {}
        },
        SystemTrayEvent::DoubleClick { .. } => {
            toggle_window_visibility(app);
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
        .manage(Db {
            connection: Default::default(),
        })
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

            let lang_dir = app.path_resolver().resolve_resource("resources").unwrap();

            let mut lang_dir = lang_dir.to_string_lossy().to_string();
            if cfg!(target_os = "windows") {
                lang_dir = lang_dir.strip_prefix(r"\\?\").unwrap().to_string();
            }
            *utils::ocr::TESSDATA_DIR.lock().unwrap() = Some(lang_dir);

            let db_path = app
                .path_resolver()
                .resolve_resource("resources/stardict.db")
                .unwrap();
            println!("data path: {:?}", db_path.to_str());
            tauri::async_runtime::block_on(async {
                let conn = SqliteConnectOptions::new()
                    .filename(db_path)
                    .connect()
                    .await
                    .unwrap();
                let db = app.state::<Db>();
                *db.connection.lock().await = Some(conn);
            });
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
        .invoke_handler(tauri::generate_handler![lookup, close_window, lookup_prs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
