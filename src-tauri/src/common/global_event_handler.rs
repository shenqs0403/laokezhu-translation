use std::env;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Shortcut, ShortcutState};
use tauri_plugin_log::log::{debug, error};
use crate::commands::get_key_value;
use crate::common::windows_manager::{close_window, create_or_show, set_position, LABEL_TRANSLATE};
use crate::dao::key_value_dao::KEY_SHORTCUT;

/// 不支持Linux wayland
pub fn register_global_shortcut(app_handle: &AppHandle) -> anyhow::Result<()>{
    // if env::var("XDG_SESSION_TYPE").is_ok() {
    //     error!("快捷键不支持wayland");
    //     return Ok(());
    // }
    let shortcut: Shortcut = load_shortcut()?.parse()?;
    debug!("shortcut: {:?}", shortcut);

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts([shortcut,"Escape".parse::<Shortcut>()?])?
        .with_handler(move |handler, hotkey, event| {
            if event.state != ShortcutState::Pressed{
                return ;
            }
            debug!("快捷键按下：{}",hotkey);
            // ESC键退出翻译窗口
            if hotkey == &Shortcut::new(None,Code::Escape) {
                close_window(handler,LABEL_TRANSLATE).ok();
                return ();
            }
            debug!("执行翻译快捷键");
            create_or_show(handler,LABEL_TRANSLATE).and_then(|win| {
                debug!("创建窗口对象");
                win.show()?;
                let pos = handler.cursor_position()?;
                set_position(handler,&win,pos)
            }).ok();
            ()
        })
        .build();
    app_handle.plugin(plugin)?;
    Ok(())
}

fn load_shortcut() -> anyhow::Result<String> {
    get_key_value(KEY_SHORTCUT.to_string())
        .map_err(anyhow::Error::from)
}