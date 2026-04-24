use crate::translators::baidu_translator::BaiduTranslator;
use crate::translators::youdao_translator::YoudaoTranslator;
use crate::dao::{engine_dao, Engine};

pub mod baidu_translator;
pub mod youdao_translator;

/// 翻译器的抽象接口
pub trait Translator {
    /// 发送http请求，返回请求结果json字符串
    async fn start_http(&mut self) -> anyhow::Result<String>;
}

/// 根据引擎名称获取翻译引擎对象
pub async  fn start_translation(engine_name: String) -> anyhow::Result<String> {
    let engine: Engine;
    if engine_name.is_empty() {
        engine = engine_dao::select_enable_engine()?;
    } else {
        engine = engine_dao::select_by_engine_name(&engine_name)?;
    }
    let result_str: String = match engine.engine_name.as_str() {
        "baidu" => BaiduTranslator::new().start_http().await?,
        "youdao" => YoudaoTranslator::new().start_http().await?,
        _ => anyhow::bail!("Unsupported engine name: {}", engine_name),
    };
    Ok(result_str)
}
