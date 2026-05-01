use base64::Engine;
use hmac::{Hmac, KeyInit, Mac};
use sha1::Sha1;
use crate::common::read_selected_text;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use tauri_plugin_log::log::debug;
use crate::dao;
use crate::translate_v1::aliyun_translator::AliyunTranslator;
use crate::translate_v1::baidu_translator::BaiduTranslator;
use crate::translate_v1::youdao_translator::YoudaoTranslator;

pub mod baidu_translator;
pub mod youdao_translator;
pub mod aliyun_translator;

pub async fn translate(_source_text: String,source_lang: String,target_lang:String,engine_name:String) -> anyhow::Result<TranslateResult> {
    let engine: dao::Engine = if engine_name.is_empty() {
        dao::engine_dao::select_enable_engine()?
    } else {
        dao::engine_dao::select_by_engine_name(&engine_name)?
    };

    // todo 代优化代码
    match engine.engine_name.as_str() {
        "baidu" => {
            let mut x = BaiduTranslator::new(engine, source_lang, target_lang);
            execute_translator(x).await
        },
        "youdao" => {
            let mut x = YoudaoTranslator::new(engine, source_lang, target_lang);
            execute_translator(x).await
        },
        "aliyun" => {
            let x = AliyunTranslator::new(engine, source_lang, target_lang);
            execute_translator(x).await
        }
        _ => anyhow::bail!("不支持的翻译引擎！！！")
    }
}

async fn execute_translator(mut trans: impl Translator) -> anyhow::Result<TranslateResult> {
    trans.prepare_data()?;
    trans.http_request().await?;
    trans.translate_result()
}

/// 统一翻译给前端界面的数据格式
#[derive(Deserialize,Serialize,Debug,Default,Clone)]
pub struct TranslateResult {
    // 是否成功
    pub success: bool,
    // 提示信息
    pub msg: String,
    // 翻译后的文本
    pub target_text: String,
    // 待翻译的文本
    pub source_text: String,
    // 目标语言
    pub target_lang: String,
    // 待翻译文本语言
    pub source_lang: String,
    // 发音音频地址
    pub video_url: String,
    // 音标
    pub symbol: String,
}

impl TranslateResult {
    pub fn error(msg: String) -> Self {
        TranslateResult{
            success: false,
            msg,
            target_text: "".to_string(),
            source_text: "".to_string(),
            target_lang: "".to_string(),
            source_lang: "".to_string(),
            video_url: "".to_string(),
            symbol: "".to_string(),
        }
    }
}

type HmacSha1 = Hmac<Sha1>;

pub trait Translator {
    // 准备数据
    fn prepare_data(&mut self) -> anyhow::Result<&Self>;
    // 获取GET请求的url
    fn get_request_url(&self) -> anyhow::Result<String>;
    // 请求返回的body处理
    fn result_json_handler(&mut self,result_json: String) -> anyhow::Result<&Self>;
    // 翻译后响应的json字符串专程专门的TranslateResult对象
    fn translate_result(self) -> anyhow::Result<TranslateResult> ;
    // 目前已知阿里云和腾讯翻译需要这个参数
    fn get_hmac_key(&self) -> String {
        "".to_string()
    }
    // 发送http请求
    async fn http_request(&mut self) -> anyhow::Result<&Self>{
        let response = tauri_plugin_http::reqwest::get(self.get_request_url()?).await?;
        if !response.status().is_success() {
            anyhow::bail!("请求异常：{}",response.status());
        }
        let string = response.text().await?;
        debug!("请求返回：{}",string);
        self.result_json_handler(string)?;
        Ok(self)
    }
    // 一个简陋的检查文本语言的方法
    fn simple_language_check(&self,text: &String) -> anyhow::Result<String> {
        if text.len() != text.chars().count() { Ok("zh-CN".to_string()) } else { Ok("en-US".to_string()) }
    }
    // 获取选中的文本，如果文本为空抛出异常停止翻译操作
    fn get_selected(&self) -> anyhow::Result<String> {
        let string = read_selected_text()?.trim().to_string();
        if string.is_empty() {
            anyhow::bail!("没有读取到选中的文本，或选中文本为空字符，请重试！");
        }
        Ok(string)
    }
    // hmac_sha1 签名
    fn hmac_sha1(&self, string_to_sign: String) -> anyhow::Result<String> {
        let mut hmac = HmacSha1::new_from_slice(self.get_hmac_key().as_bytes())?;
        hmac.update(string_to_sign.as_bytes());
        let result = hmac.finalize().into_bytes();
        Ok(STANDARD.encode(&result))
    }
}