pub mod database_manager;
pub mod tray_icon_manager;

use rust_i18n::t;
use tauri::AppHandle;
use tauri_plugin_log::log::{debug, error, info, LevelFilter};
use tauri_plugin_log::{Target, TargetKind};
use crate::common::database_manager::init_database;
use crate::common::tray_icon_manager::init_tray_icon;

/// 当前 common 模块的初始化
pub fn init(app_handle: &AppHandle) -> tauri::Result<()> {
    init_log(app_handle)?;
    let result = init_database(app_handle).and_then(|_| {
        Ok(())
    });
    if result.is_err() {
        info!("{}", result.unwrap_err());
    }
    init_tray_icon(app_handle)?;
    Ok(())
}

/// 初始化日志
fn init_log(app_handle: &AppHandle) -> tauri::Result<()> {
    // 日志，开发环境使用debug,其他环境使用error
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Error;

    // 开发环境添加webview控制台输出到控制台，其他环境输出到文件
    let target = if cfg!(debug_assertions) {
        Target::new(TargetKind::Webview)
    } else {
        Target::new(TargetKind::LogDir {file_name: None})
    };

    let plugin = tauri_plugin_log::Builder::new()
        .target(target)
        .build();
    app_handle.plugin(plugin)
}