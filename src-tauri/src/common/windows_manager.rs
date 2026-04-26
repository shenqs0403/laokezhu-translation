use tauri::{AppHandle, Manager, PhysicalPosition, Position, WebviewWindow, WebviewWindowBuilder};
use tauri_plugin_log::log::debug;

pub static LABEL_TRANSLATE: &str = "translate";
pub static LABEL_CONFIG: &str = "config";
pub static LABEL_ABOUT: &str = "about";

pub fn close_window(app_handle: &AppHandle,label: &str) -> anyhow::Result<()> {
    let option = app_handle.get_webview_window(label);
    if let Some (win) = option {
        win.close()?;
    }
    Ok(())
}

/// 根据label创建窗口，如果窗口存在就显示
pub fn create_or_show(app_handle: &AppHandle, label: &str) -> anyhow::Result<WebviewWindow> {
    let option = app_handle.get_webview_window(label);
    if option.is_some() {
        debug!("找到窗口对象，直接返回");
        return Ok(option.unwrap());
    }
    let option = app_handle.config().app.windows.iter()
        .filter(|x| x.label == label)
        .next();
    debug!("找到窗口对象配置信息:{:?}",option);
    if option.is_none() {
        anyhow::bail!("Could not find window {}", label);
    }
    WebviewWindowBuilder::from_config(app_handle, option.unwrap())?
        .build()
        .map_err(anyhow::Error::from)
}

pub fn set_position(app_handle: &AppHandle, win: &WebviewWindow, mut position: PhysicalPosition<f64>) -> anyhow::Result<()> {
    position.x += 10.0;
    position.y += 10.0;
    win.set_position(position)?;
    Ok(())
}
