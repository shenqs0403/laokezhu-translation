use std::process::exit;
use rust_i18n::t;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder};
use tauri::AppHandle;

// 系统托盘
pub fn init_tray_icon(app_handle: &AppHandle) -> anyhow::Result<()> {
    let tray_icon = create_tray(app_handle)?;
    init_menu_item_event(&tray_icon);
    Ok(())
}

fn create_tray(app_handle: &AppHandle) -> tauri::Result<TrayIcon> {
    let quit_item = MenuItem::with_id(app_handle, "quit", t!("tray.quit"), true, None::<&str>)?;
    let config_item = MenuItem::with_id(app_handle, "config", t!("tray.config.label"), true, None::<&str>)?;
    let about_item = MenuItem::with_id(app_handle, "about", t!("tray.about.label"), true, None::<&str>)?;
    let menu = Menu::with_items(app_handle, &[&config_item,&about_item,&quit_item])?;
    let tray_icon = TrayIconBuilder::new()
        .menu(&menu)
        .build(app_handle)?;
    Ok(tray_icon)
}

fn init_menu_item_event(tray_icon: &TrayIcon) {
    tray_icon.on_menu_event(|app_handle: &AppHandle, event| {
        match event.id.0.as_ref() {
            "config" => {},
            "about" => {},
            "quit" => {
                exit(0);
            },
            _ => {}
        }
    })
}