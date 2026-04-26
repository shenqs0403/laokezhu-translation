use std::env;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_log::log::error;
use crate::commands::get_key_value;
use crate::common::windows_manager::{create_or_show, set_position};
use crate::dao::key_value_dao::KEY_SHORTCUT;

/// 不支持Linux wayland
pub fn register_global_shortcut(app_handle: &AppHandle) -> anyhow::Result<()>{
    if env::var("XDG_SESSION_TYPE").is_ok() {
        error!("快捷键不支持wayland");
        return Ok(());
    }
    let shortcut: Shortcut = load_shortcut()?.parse()?;
    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(shortcut)?
        .with_handler(move |handler, hotkey, event| {
            if event.state != ShortcutState::Pressed || hotkey != &shortcut{
                return ;
            }
            create_or_show(handler,"translation").and_then(|win| {
                let pos = handler.cursor_position()?;
                set_position(handler,&win,pos)
            }).ok();
            ()
        })
        .build();
    app_handle.plugin(plugin)?;
    app_handle.global_shortcut().register(shortcut)?;
    Ok(())
}

fn load_shortcut() -> anyhow::Result<String> {
    get_key_value(KEY_SHORTCUT.to_string())
        .map_err(anyhow::Error::from)
}