use std::string::ToString;
use std::sync::Mutex;
use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde_json::Value;
use sha2::digest::typenum::private::Trim;
use tauri::AppHandle;
use tauri_plugin_log::log::debug;
use crate::common::read_selected_text;
use crate::common::windows_manager::{create_or_show, set_position, LABEL_MENU};

lazy_static! {
    static ref STOP_FLAG: Mutex<bool> = Mutex::new(false);
    static ref POLL_TIME: Mutex<u64> = Mutex::new(3000);
    static ref LAST_TEXT: Mutex<String> = Mutex::new(String::new());
}

/// 启动轮询
pub fn start_poll(app_handle: AppHandle) -> anyhow::Result<()> {
    std::thread::spawn(move || -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;
        while !get_stop_flag() {
            std::thread::sleep(std::time::Duration::from_millis(get_poll_time()));
            let string = clipboard.get().clipboard(LinuxClipboardKind::Primary).text()?;
            if !compare_last_text(string)? {
                let window = create_or_show(&app_handle, LABEL_MENU)?;
                window.show()?;
                set_position(&window,app_handle.cursor_position()?)?;
            }
        }
        Ok(())
    });
    Ok(())
}

fn compare_last_text(string: String) -> anyhow::Result<bool> {
    let string = string.trim();
    if string.is_empty() {
        return Ok(true);
    }
    let mut guard = LAST_TEXT.lock().unwrap();
    if guard.is_empty() {
        *guard = string.to_string();
        return Ok(true);
    }
    if guard.eq(string) {
        return Ok(true);
    }
    *guard = string.to_string();
    Ok(false)
}

pub fn set_poll_time(time: u64) -> anyhow::Result<()> {
    let mut guard = POLL_TIME.lock().unwrap();
    *guard = time;
    Ok(())
}

fn get_poll_time() -> u64 {
    POLL_TIME.lock().unwrap().clone()
}

fn get_stop_flag() -> bool {
    STOP_FLAG.lock().unwrap().clone()
}