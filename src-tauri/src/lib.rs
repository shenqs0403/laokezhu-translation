mod common;
mod dao;
mod translators;
mod commands;

use tauri_plugin_log::log::{error, warn};
use commands::{ get_all_engines,save_key_value,get_key_value,save_engine,is_wayland,update_shortcut,translate_selected_text,open_translate_window};
use crate::common::windows_manager::{create_or_show, set_position, LABEL_TRANSLATE};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handler = app.handle();
            common::init(app_handler)?;
            dao::init().unwrap_or_else(|e| {
                warn!("数据库初始化出了点文件，如果是表已经存在问题则忽律")
            });
            // 必须在dao初始化完成后使用
            common::global_event_handler::start_shortcut_handler(app_handler.clone())?;
            common::tray_manager::init(app_handler)?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app_handler, args, _cwd| {
            if args.contains(&"translate".to_string()) {
                create_or_show(app_handler, LABEL_TRANSLATE).and_then(|w| {
                    w.set_focus()?;
                    set_position(&w,app_handler.cursor_position()?)
                }).unwrap_or_else(|e| {
                    error!("打开翻译窗口异常：{}",e)
                });
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_engines,save_key_value,update_shortcut,get_key_value,save_engine,is_wayland,translate_selected_text,open_translate_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
