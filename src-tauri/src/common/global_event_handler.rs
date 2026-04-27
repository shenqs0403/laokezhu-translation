use crate::commands::get_key_value;
use crate::common::windows_manager::{close_window, create_or_show, set_position, LABEL_MENU, LABEL_TRANSLATE};
use crate::dao::key_value_dao::KEY_SHORTCUT;
use rdev::{Button, EventType};
use std::env;
use tauri::{AppHandle, Manager, PhysicalPosition};
use tauri_plugin_global_shortcut::{Code, Shortcut, ShortcutState};
use tauri_plugin_log::log::{debug, error};

#[derive(Default,Debug)]
pub struct MousePosition {
    pub start_x: f64,
    pub start_y: f64,
    pub real_x: f64,
    pub real_y: f64,
}
impl MousePosition {
    pub fn record_position(&mut self) {
        self.start_x = self.real_x;
        self.start_y = self.real_y;
    }
}

/// 注册全局鼠标事件，这个方法在Linux Wayland环境无效
pub fn register_mouse_event(app_handle: &AppHandle) -> anyhow::Result<()> {
    debug!("{:?}",env::var("XDG_SESSION_TYPE"));
    if env::var("WAYLAND_DISPLAY").is_ok() {
        error!("鼠标事件不支持Wayland");
        return Ok(());
    }
    let handler = app_handle.clone();
    std::thread::spawn(move || {
        let mut pos = MousePosition::default();
        rdev::listen(move |event| match event.event_type {
            EventType::ButtonPress(Button::Left) => {
                pos.record_position();
            }
            EventType::ButtonRelease(Button::Left) => {
                if (pos.start_x - pos.real_x).abs() > 10.0
                    || (pos.start_y - pos.real_y).abs() > 10.0
                {
                    let window = create_or_show(&handler, LABEL_MENU).unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    set_position(&window,PhysicalPosition::new(pos.real_x, pos.real_y)).unwrap();
                }
            }
            EventType::MouseMove { x, y } => {
                pos.real_x = x;
                pos.real_y = y;
            }
            _ => {}
        })
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
