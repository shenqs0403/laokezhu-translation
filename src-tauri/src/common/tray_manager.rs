use std::process::exit;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;
use tauri_plugin_log::log::warn;
use crate::common::windows_manager::{create_or_show, LABEL_ABOUT, LABEL_CONFIG};

pub fn init(app_handle: &AppHandle) -> anyhow::Result<()> {
    let quit_item = MenuItem::with_id(app_handle, "quit", "Quit", true, None::<&str>)?;
    let config_item = MenuItem::with_id(app_handle, "config", "Config", true, None::<&str>)?;
    let about_item = MenuItem::with_id(app_handle, "about", "About", true, None::<&str>)?;
    let menu = Menu::with_items(app_handle, &[&config_item, &about_item, &quit_item])?;
    let icon = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|handler, event| {
            let _ = match event.id.as_ref() {
                "quit" => exit(1),
                "config" => create_or_show(handler,LABEL_CONFIG).ok(),
                "about" => create_or_show(handler,LABEL_ABOUT).ok(),
                _ => {
                    warn!("不支持的系统托盘操作");
                    None
                },
            };
        })
        .build(app_handle)?;
    Ok(())
}
