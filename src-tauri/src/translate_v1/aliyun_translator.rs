use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use tauri_plugin_log::log::debug;
use crate::dao::Engine;
use crate::translate_v1::{TranslateResult, Translator};

#[derive(Serialize,Deserialize,Debug,Default,Clone)]
struct AliyunTransDataResult {
    #[serde(rename="Translated")]
    translated: String,
}
#[derive(Serialize,Deserialize,Debug,Default,Clone)]
struct AliyunResult {
    #[serde(rename = "Code")]
    code: i32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Data")]
    data: AliyunTransDataResult
}

pub struct AliyunTranslator {
    engine:Engine,
    source_lang: String,
    target_lang: String,
    request_url: String,
    params: BTreeMap<String, String>,
    result_json: Option<AliyunResult>,
}

impl AliyunTranslator {
    pub fn new(engine: Engine,source_lang: String, target_lang: String) -> AliyunTranslator {
        AliyunTranslator{
            engine,
            source_lang,
            target_lang,
            request_url: "".to_string(),
            params: BTreeMap::new(),
            result_json: None,
        }
    }

    fn default_target_lang(&self) -> anyhow::Result<String> {
        if !self.target_lang.is_empty() {
            return Ok(self.target_lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if ["zh-HK","zh-SG","zh-TW"].contains(&string.as_str()) {
            return Ok("zh-tw".to_string());
        }
        Ok(string.split("-").next().unwrap().to_string())
    }
}

impl Translator for AliyunTranslator {
    fn prepare_data(&mut self) -> anyhow::Result<&Self> {
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() {
            anyhow::bail!("请正确配置《阿里云》翻译引擎！！！")
        }
        let text = self.get_selected()?;
        self.params.insert("Action".to_string(),"TranslateGeneral".to_string());
        self.params.insert("Version".to_string(),"2018-10-12".to_string());
        self.params.insert("Format".to_string(),"JSON".to_string());
        self.params.insert("AccessKeyId".to_string(),"LTAI5t7TG5EXNF5F5DC8aroj".to_string());
        self.params.insert("SignatureNonce".to_string(),rand::random::<u32>().to_string());
        self.params.insert("Timestamp".to_string(),chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        self.params.insert("SignatureMethod".to_string(),"HMAC-SHA1".to_string());
        self.params.insert("SignatureVersion".to_string(),"1.0".to_string());
        self.params.insert("FormatType".to_string(),"text".to_string());
        self.params.insert("Scene".to_string(),"general".to_string());
        self.params.insert("SourceLanguage".to_string(),self.simple_language_check(&text)?);
        self.params.insert("SourceText".to_string(),text);
        self.params.insert("TargetLanguage".to_string(),self.default_target_lang()?);

        let param_str = self.params.iter().map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<String>>().join("&");

        let string_to_sign = format!("GET&{}&{}",urlencoding::encode("/"),urlencoding::encode(&param_str));
        debug!("本地签名的字符串：{}",string_to_sign);
        let sign = self.hmac_sha1(string_to_sign)?;
        let url = format!("{}?{}&Signature={}",self.engine.url,param_str,urlencoding::encode(&sign));
        debug!("阿里云翻译url:{}",url);
        self.request_url = url;
        todo!()
    }

    fn get_request_url(&self) -> anyhow::Result<String> {
        Ok(self.request_url.clone())
    }

    fn result_json_handler(&mut self, result_json: String) -> anyhow::Result<&Self> {
        self.result_json = Some(serde_json::from_str::<AliyunResult>(&result_json)?);
        Ok(self)
    }

    fn translate_result(self) -> anyhow::Result<TranslateResult> {
        let result = self.result_json.unwrap();
        Ok(TranslateResult {
            success: true,
            msg: if result.code == 200 { "操作成功".to_string() } else { result.message },
            target_text: result.data.translated,
            source_text: "".to_string(),
            target_lang: self.target_lang,
            source_lang: self.source_lang,
            video_url: "".to_string(),
            symbol: "".to_string(),
        })
    }
}