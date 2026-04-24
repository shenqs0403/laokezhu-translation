use crate::dao::engine_dao::select_all_engine;
use crate::dao::Engine;
use tauri::command;

#[command]
pub fn get_all_engines() -> tauri::Result<Vec<Engine>> {
    let engines = select_all_engine()?;
    Ok(engines)
}

#[command]
pub fn save_key_value(key: String, value: String) -> tauri::Result<()> {
    save_key_value(key, value)?;
    Ok(())
}
