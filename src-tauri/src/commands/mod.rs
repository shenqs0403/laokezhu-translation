use std::env;
use std::env::var;
use anyhow::anyhow;
use crate::dao::engine_dao::select_all_engine;
use crate::dao::{Engine};
use tauri::{command, AppHandle, Manager};
use tauri_plugin_log::log::{debug, error};
use crate::common::windows_manager::{create_or_show, set_position, LABEL_MENU, LABEL_TRANSLATE};
use crate::{common, dao};
use crate::common::global_event_handler::restart_shortcut_handler;
use crate::dao::key_value_dao::{get_item, set_item, KEY_SHORTCUT, KEY_SWIPE};
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
    debug!("修改引擎：{:?}",engine);
    let i = dao::engine_dao::save_engine(engine)?;
    Ok(i)
}

#[command]
pub fn update_shortcut(app_handle: AppHandle,value: String) -> tauri::Result<()> {
    set_item(KEY_SHORTCUT.to_string(),value)?;
    restart_shortcut_handler(app_handle)?;
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
