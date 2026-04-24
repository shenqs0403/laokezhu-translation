use rust_i18n::t;
use tauri_plugin_log::log::{debug, error, info, warn};

rust_i18n::i18n!("i18n");

mod common;
mod dao;
mod translators;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            common::init(app.handle())?;
            dao::init()?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|_app_handler, _args, _cwd| {}

        ))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
