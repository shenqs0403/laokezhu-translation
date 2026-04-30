use base64::Engine;
use hmac::{Hmac, KeyInit, Mac};
use tauri_plugin_log::log::debug;
use crate::translators::baidu_translator::BaiduTranslator;
use crate::translators::youdao_translator::YoudaoTranslator;
use crate::dao::{engine_dao};
use base64::engine::general_purpose::STANDARD;
use crate::translators::aliyun_translator::AliyunTranslator;
use crate::translators::tencent_translator::TencentTranslator;

pub mod baidu_translator;
pub mod youdao_translator;
pub mod aliyun_translator;
pub mod tencent_translator;

type HmacSha1 = Hmac<sha1::Sha1>;

/// 翻译器的抽象接口
pub trait Translator {
    /// 发送http请求，返回请求结果json字符串
    async fn start_http(&mut self) -> anyhow::Result<String>;

    fn get_key(&self) -> String {
        "".to_string()
    }

    fn hmac_sha1(&self,sign_string: String) -> anyhow::Result<String> {
        let string = self.get_key();
        debug!("Hmac-Sha1: {}",string);
        let mut hmac = HmacSha1::new_from_slice(self.get_key().as_bytes())?;
        hmac.update(sign_string.as_bytes());
        let x = STANDARD.encode(hmac.finalize().into_bytes());
        Ok(x)
    }
}

/// 根据引擎名称获取翻译引擎对象
pub async  fn start_translation(engine_name: String, lang: String) -> anyhow::Result<String> {
    let engine;
    if engine_name.is_empty() {
        engine = engine_dao::select_enable_engine()?;
    } else {
        engine = engine_dao::select_by_engine_name(&engine_name)?;
    }
    debug!("查询引擎：{:?} ", engine);
    let result_str: String = match engine.engine_name.as_str() {
        "baidu" => BaiduTranslator::new(engine,lang).start_http().await?,
        "youdao" => YoudaoTranslator::new(engine,lang).start_http().await?,
        "aliyun" => AliyunTranslator::new(engine,lang).start_http().await?,
        "tencent" => TencentTranslator::new(engine,lang).start_http().await?,
        _ => anyhow::bail!("Unsupported engine name: {}", engine_name),
    };
    Ok(result_str)
}
