use crate::commands::get_key_value;
use crate::common::windows_manager::{create_or_show, set_position, LABEL_MENU, LABEL_TRANSLATE};
use crate::dao::key_value_dao::{get_item, set_item, KEY_SHORTCUT, KEY_SWIPE};
use std::env;
use std::sync::Mutex;
use lazy_static::lazy_static;
use tauri::{AppHandle};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};
use tauri_plugin_log::log::{debug, error};
use crate::common;

lazy_static! {
    pub static ref READ_TEXT: Mutex<String> = Mutex::new(String::new());
    static ref SWIPE_TIME: Mutex<u64> = Mutex::new(300);
    static ref STOP_FLAG: Mutex<bool> = Mutex::new(false);
}

/// 注册全局鼠标事件，这个方法在Linux Wayland环境无效
pub fn register_mouse_event(app_handle: &AppHandle) -> anyhow::Result<()> {
    load_swipe_time_from_db()?;
    let handler = app_handle.clone();
    std::thread::spawn(move || -> anyhow::Result<()> {
        while !get_stop_flag()? {
            let i = get_swipe_time()?;
            if i < 100 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(i));
            let string = common::read_selected_text()?.trim().to_string();
            debug!("读取剪切板内容：{}",string);
            if string.is_empty() {
                continue;
            }

            let x = !compare_text(&string);
            debug!("比较结果：{}",x);
            if !compare_text(&string) {
                set_text(string)?;
                let window = create_or_show(&handler, LABEL_MENU)?;
                // window.set_always_on_top(true)?;
                set_position(&window,handler.cursor_position()?)?;
            }
        }
        debug!("退出轮询");
        Ok(())
    });
    Ok(())
}

/// 不支持Linux wayland
pub fn register_global_shortcut(app_handle: &AppHandle) -> anyhow::Result<()> {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        error!("快捷键不支持Wayland");
        return Ok(());
    }
    let shortcut: Shortcut = load_shortcut()?.parse()?;
    debug!("shortcut: {:?}", shortcut);

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts([shortcut])?
        .with_handler(move |handler, hotkey, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            debug!("快捷键按下：{}", hotkey);
            create_or_show(handler, LABEL_TRANSLATE)
                .and_then(|win| {
                    debug!("创建窗口对象");
                    win.show()?;
                    win.set_focus()?;
                    let pos = handler.cursor_position()?;
                    set_position(&win, pos)
                })
                .ok();
            ()
        })
        .build();
    app_handle.plugin(plugin)?;
    Ok(())
}

fn load_shortcut() -> anyhow::Result<String> {
    get_key_value(KEY_SHORTCUT.to_string()).map_err(anyhow::Error::from)
}

fn set_text(text: String) -> anyhow::Result<()> {
    let mut guard = READ_TEXT.lock().unwrap();
    debug!("设置前：{}",&guard);
    *guard = text;
    debug!("设置后：{}",&guard);
    Ok(())
}

pub fn set_swipe_time(time: u64) -> anyhow::Result<()> {
    let mut guard = SWIPE_TIME.lock().unwrap();
    *guard = time;
    Ok(())
}

fn get_swipe_time() -> anyhow::Result<u64> {
    let mut guard = SWIPE_TIME.lock().unwrap();
    Ok(*guard)
}

pub fn load_swipe_time_from_db() -> anyhow::Result<u64> {
    let i = get_item(KEY_SWIPE.to_string())?.value.parse::<u64>()?;
    let mut guard = SWIPE_TIME.lock().unwrap();
    *guard = i;
    Ok(i)
}

/// 减小lock的范围
fn compare_text(new_str: &str) -> bool {
    READ_TEXT.lock().unwrap().as_str() == new_str
}

fn get_stop_flag() -> anyhow::Result<bool> {
    Ok(STOP_FLAG.lock().unwrap().clone())
}