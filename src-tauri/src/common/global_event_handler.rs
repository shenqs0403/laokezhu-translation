use std::sync::{Mutex};
use lazy_static::lazy_static;
use tauri::{AppHandle};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_log::log::{debug, error, warn};
use crate::common::windows_manager::{create_or_show, set_position, LABEL_MENU, LABEL_TRANSLATE};
use crate::dao::key_value_dao::{get_item, KEY_SHORTCUT};

lazy_static! {
    static ref SHORTCUT: Mutex<String> = Mutex::new(String::new());
}

/// 启动快捷键
/// 用户更换的时候直接替换快捷键
/// 这个只在非Linux系统有效
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn start_shortcut_handler(app_handle: AppHandle) -> anyhow::Result<()> {
    let shortcut = get_item(KEY_SHORTCUT.to_string())?.value;
    debug!("启用的快捷键：{}",shortcut);
    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|handler, hotkey, event| {
            if event.state == ShortcutState::Pressed {
                let window_result = create_or_show(handler, LABEL_TRANSLATE);
                if window_result.is_err() {
                    error!("{}", window_result.err().unwrap());
                } else {
                    let win = window_result.unwrap();
                    win.show().unwrap();
                    win.set_always_on_top(true).unwrap();
                    set_position(&win,handler.cursor_position().unwrap()).ok();
                }
            }
        })
        .build();
    set_shortcut_id(shortcut.clone())?;
    app_handle.plugin(plugin)?;
    app_handle.global_shortcut().register(shortcut.parse::<Shortcut>()?)?;
    Ok(())
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
/// 重新监听快捷键，先解绑快捷键在重新绑定
pub fn restart_shortcut_handler(app_handle: AppHandle) -> anyhow::Result<()> {
    unlisten_shortcut(&app_handle)?;
    start_shortcut_handler(app_handle)?;
    Ok(())
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn unlisten_shortcut(app_handle: &AppHandle) -> anyhow::Result<()> {
    let guard = SHORTCUT.lock().unwrap();
    app_handle.global_shortcut().unregister(guard.parse::<Shortcut>()?)?;
    Ok(())
}

/// 设置全局id,便于注销操作
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn set_shortcut_id(shortcut: String) -> anyhow::Result<()> {
    let mut guard = SHORTCUT.lock().unwrap();
    *guard = shortcut;
    Ok(())
}