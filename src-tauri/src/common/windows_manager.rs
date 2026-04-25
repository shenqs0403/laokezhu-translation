use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder};

/// 根据label创建窗口，如果窗口存在就显示
pub fn create_or_show(app_handle: &AppHandle,label: &str) -> anyhow::Result<WebviewWindow> {
    let option = app_handle.get_webview_window(label);
    if option.is_some() {
        return Ok(option.unwrap());
    }
    let option = app_handle.config().app.windows.iter().filter(|x| x.label == label)
        .next();
    if option.is_none() {
        anyhow::bail!("Could not find window {}", label);
    }
    WebviewWindowBuilder::from_config(app_handle,option.unwrap())?.build()
        .map_err(anyhow::Error::from)
}