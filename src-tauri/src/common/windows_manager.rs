use tauri::{AppHandle, Manager, PhysicalPosition, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_log::log::{debug, error};

pub static LABEL_TRANSLATE: &str = "translate";
pub static LABEL_CONFIG: &str = "config";
pub static LABEL_ABOUT: &str = "about";
pub static LABEL_MENU: &str = "menu";

pub fn close_window(app_handle: &AppHandle,label: &str) -> anyhow::Result<()> {
    let option = app_handle.get_webview_window(label);
    if let Some (win) = option {
        win.close()?;
    }
    Ok(())
}

/// 根据label创建窗口，如果窗口存在就显示
pub fn create_or_show(app_handle: &AppHandle, label: &str) -> anyhow::Result<WebviewWindow> {
    debug!("Creating window: {}", label);
    let option = app_handle.get_webview_window(label);
    let win = match option {
        None => {
            let config_option = app_handle.config().app.windows.iter()
                .filter(|x| x.label == label)
                .next();
            debug!("当前线程：{:?}",std::thread::current().name());
            WebviewWindowBuilder::from_config(app_handle, config_option.unwrap())?
                .build()
                .map_err(|e| {
                    error!("{}", e);
                    anyhow::Error::new(e)
                })
        }
        Some(win) => {Ok(win)}
    }?;
    win.show()?;
    // win.set_focus()?;
    Ok(win)
}

pub fn set_position(win: &WebviewWindow, mut position: PhysicalPosition<f64>) -> anyhow::Result<()> {
    position.x += 10.0;
    position.y += 10.0;
    win.set_position(position)?;
    Ok(())
}
