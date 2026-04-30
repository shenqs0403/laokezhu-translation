use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use rdev::{EventType, Key};
use tauri_plugin_log::log::debug;
use crate::translators::baidu_translator::BaiduTranslator;
use crate::translators::youdao_translator::YoudaoTranslator;
use crate::dao::{engine_dao, Engine};

pub mod baidu_translator;
pub mod youdao_translator;
pub mod aliyun_translator;
pub mod tencent_translator;

/// 翻译器的抽象接口
pub trait Translator {
    /// 发送http请求，返回请求结果json字符串
    async fn start_http(&mut self) -> anyhow::Result<String>;
}

/// 根据引擎名称获取翻译引擎对象
pub async  fn start_translation(engine_name: String, lang: String) -> anyhow::Result<String> {
    let engine: Engine;
    if engine_name.is_empty() {
        engine = engine_dao::select_enable_engine()?;
    } else {
        engine = engine_dao::select_by_engine_name(&engine_name)?;
    }
    debug!("查询引擎：{:?} ", engine);
    let result_str: String = match engine.engine_name.as_str() {
        "baidu" => BaiduTranslator::new(engine,lang).start_http().await?,
        "youdao" => YoudaoTranslator::new(engine,lang).start_http().await?,
        _ => anyhow::bail!("Unsupported engine name: {}", engine_name),
    };
    Ok(result_str)
}
