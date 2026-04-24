use tauri::AppHandle;
use tauri_plugin_log::log::LevelFilter;
use tauri_plugin_log::{Target, TargetKind};

/// 当前 common 模块的初始化
pub fn init(app_handle: &AppHandle) -> tauri::Result<()> {
    init_log(app_handle)?;
    Ok(())
}

/// 初始化日志
fn init_log(app_handle: &AppHandle) -> tauri::Result<()> {
    // 日志，开发环境使用debug,其他环境使用error
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Error;

    // 开发环境添加webview控制台输出到控制台，其他环境输出到文件
    let target = if cfg!(debug_assertions) {
        Target::new(TargetKind::Webview)
    } else {
        Target::new(TargetKind::LogDir {file_name: None})
    };

    let plugin = tauri_plugin_log::Builder::new()
        .target(target)
        .build();
    app_handle.plugin(plugin)
}

/// 获取系统本地语言代码
pub fn get_locale() -> anyhow::Result<String> {
    let string = sys_locale::get_locale().unwrap_or_else(|| "en_US".to_string());
    Ok(string)
}