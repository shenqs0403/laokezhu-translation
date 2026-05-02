mod commands;
mod common;
mod dao;
mod translate_v1;

use std::env;
use crate::common::windows_manager::{create_or_show, set_position, LABEL_TRANSLATE};
use commands::{
    get_all_engines, get_key_value, is_wayland, open_translate_window, save_engine, save_key_value,
    translate_selected_text,
    update_swipe
};
use tauri_plugin_log::log::{error, warn};
use crate::dao::upgrade;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            std::fs::create_dir_all(env::home_dir().unwrap().join(".config").join("laokezhu")).unwrap();
            let app_handler = app.handle();
            common::init(app_handler)?;
            dao::init().unwrap_or_else(|e| warn!("数据库初始化出了点文件，如果是表已经存在问题则忽律"));
            upgrade()?;
            // 必须在dao初始化完成后使用
            #[cfg(any(target_os = "windows", target_os = "macos"))]
            common::global_event_handler::start_shortcut_handler(app_handler.clone())?;

            // common::poll_event_handler::start_poll(app_handler.clone())?;
            common::tray_manager::init(app_handler)?;
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(
            |app_handler, args, _cwd| {
                if args.contains(&"translate".to_string()) {
                    create_or_show(app_handler, LABEL_TRANSLATE)
                        .and_then(|w| {
                            w.set_focus()?;
                            set_position(&w, app_handler.cursor_position()?)
                        })
                        .unwrap_or_else(|e| error!("打开翻译窗口异常：{}", e));
                }
            },
        ))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all_engines,
            save_key_value,
            #[cfg(any(target_os = "windows", target_os = "macos"))]
            commands::update_shortcut,
            get_key_value,
            save_engine,
            update_swipe,
            is_wayland,
            translate_selected_text,
            open_translate_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
