use std::env;
use crate::dao::engine_dao::select_all_engine;
use crate::dao::{Engine};
use tauri::{command, AppHandle};
use tauri_plugin_log::log::{debug};
use crate::common::windows_manager::{create_or_show, set_position, LABEL_TRANSLATE};
use crate::{common, dao};
use crate::dao::key_value_dao::{get_item, set_item, KEY_SWIPE};
use crate::translators::start_translation;

/// 划词菜单点击翻译专门提供的方法
#[command]
pub fn open_translate_window(app_handle: AppHandle) -> tauri::Result<()> {
    let window = create_or_show(&app_handle, LABEL_TRANSLATE)?;
    set_position(&window,app_handle.cursor_position()?)?;
    Ok(())
}

#[command]
pub async fn translate_selected_text(engine_name: String,lang: String) -> tauri::Result<String> {
    debug!("接收参数：{}  {}",engine_name,lang);
    let result_str = start_translation(engine_name, lang).await?;
    Ok(result_str)
}

#[command]
pub fn is_wayland() -> tauri::Result<bool> {
    Ok(env::var("WAYLAND_DISPLAY").is_ok())
}

#[command]
pub fn get_all_engines() -> tauri::Result<Vec<Engine>> {
    let engines = select_all_engine()?;
    Ok(engines)
}

#[command]
pub fn save_engine(engine: Engine) -> tauri::Result<usize> {
    let i = dao::engine_dao::save_engine(engine)?;
    Ok(i)
}

#[command]
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn update_shortcut(app_handle: AppHandle,value: String) -> tauri::Result<()> {
    common::global_event_handler::restart_shortcut_handler(app_handle)?;
    set_item(KEY_SHORTCUT.to_string(),value)?;
    Ok(())
}

#[command]
pub fn update_swipe(app_handle: AppHandle,value: String) -> tauri::Result<()> {
    set_item(KEY_SWIPE.to_string(),value.clone())?;
    let i: u64 = value.parse().unwrap_or_else(|e| 300);
    common::poll_event_handler::set_poll_time(i)?;
    Ok(())
}

#[deprecated]
#[command]
pub fn save_key_value(key: String, value: String) -> tauri::Result<()> {
    set_item(key, value.clone())?;
    Ok(())
}

#[command]
pub fn get_key_value(key: String) -> tauri::Result<String> {
    let value = get_item(key)?;
    Ok(value.value)
}
