use std::env;
use crate::dao::engine_dao::select_all_engine;
use crate::dao::{Engine};
use tauri::command;
use tauri_plugin_log::log::{debug, error};
use crate::dao;
use crate::dao::key_value_dao::{get_item, set_item};
use crate::translators::start_translation;

#[command]
pub async fn translate_selected_text(engine_name: String,lang: String) -> tauri::Result<String> {
    debug!("接收参数：{}  {}",engine_name,lang);
    let result_str = start_translation(engine_name, lang).await?;
    Ok(result_str)
}

#[command]
pub fn is_wayland() -> tauri::Result<bool> {
    Ok(env::var("XDG_SESSION_TYPE").is_ok())
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
pub fn save_key_value(key: String, value: String) -> tauri::Result<()> {
    set_item(key, value)?;
    Ok(())
}

#[command]
pub fn get_key_value(key: String) -> tauri::Result<String> {
    let value = get_item(key)?;
    Ok(value.value)
}
