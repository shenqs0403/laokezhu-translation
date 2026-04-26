mod common;
mod dao;
mod translators;
mod commands;

use tauri_plugin_log::log::{debug, warn};
use commands::{ get_all_engines,save_key_value,get_key_value,save_engine};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handler = app.handle();
            common::init(app_handler)?;
            dao::init().and_then(|t| {
                debug!("首次运行");
                Ok(())
            }).unwrap_or_else(|e| {
                warn!("数据库初始化出了点文件，如果是表已经存在问题则忽律")
            });
            common::global_event_handler::register_global_shortcut(app_handler)?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|_app_handler, _args, _cwd| {}

        ))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_engines,save_key_value,get_key_value,save_engine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
