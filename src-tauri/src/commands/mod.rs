use crate::dao::engine_dao::select_all_engine;
use crate::dao::Engine;
use tauri::command;
use crate::dao::key_value_dao::{get_item, set_item};

#[command]
pub fn get_all_engines() -> tauri::Result<Vec<Engine>> {
    let engines = select_all_engine()?;
    Ok(engines)
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
