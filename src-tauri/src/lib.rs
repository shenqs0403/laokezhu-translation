use rust_i18n::t;
use tauri_plugin_log::log::{debug, error, info, warn};

rust_i18n::i18n!("i18n");

mod common;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let string = sys_locale::get_locale().unwrap_or_else(|| "en_US".to_string());
            rust_i18n::set_locale(&string);
            common::init(app.handle())?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|_app_handler, _args, _cwd| {}

        ))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
