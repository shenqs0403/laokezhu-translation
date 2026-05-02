use std::collections::BTreeMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use tauri_plugin_log::log::debug;
use crate::dao;
use crate::translate_v1::{TranslateResult, Translator};

#[derive(Serialize,Deserialize,Debug,Default,Clone)]
struct TencentTransResponseError {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}
#[derive(Serialize,Deserialize,Debug,Default,Clone)]
struct TencentTransResponse {
    #[serde(rename = "RequestId")]
    request_id: String,
    #[serde(rename = "TotalCount")]
    total_count: Option<i32>,
    #[serde(rename = "InstanceStatusSet")]
    instance_status_set: Option<Vec<String>>,
    #[serde(rename = "Error")]
    error: Option<TencentTransResponseError>,
    #[serde(rename = "Source")]
    source: Option<String>,
    #[serde(rename = "Target")]
    target: Option<String>,
    #[serde(rename = "TargetText")]
    target_text: Option<String>,
}
#[derive(Serialize,Deserialize,Debug,Default,Clone)]
struct TencentResult {
    #[serde(rename = "Response")]
    response: TencentTransResponse,
}

pub struct TencentTranslator {
    pub engine: dao::Engine,
    pub source_lang: String,
    pub target_lang: String,
    pub request_url: String,
    pub result_json: Option<TencentResult>,
}

impl TencentTranslator {
    pub fn new(engine: dao::Engine ,source_lang: String, target_lang: String) -> TencentTranslator {
        TencentTranslator{ engine,source_lang,target_lang, request_url: "".to_string(), result_json: None }
    }

    fn get_target_lang(&self) -> anyhow::Result<String> {
        if !self.target_lang.is_empty() {
            return Ok(self.target_lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if "zh-TW".eq(&string) {
            return Ok(string);
        }
        Ok(string.split("-").next().unwrap().to_string())
    }
}

impl Translator for TencentTranslator {
    fn prepare_data(&mut self) -> anyhow::Result<&Self> {
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() || self.engine.region.is_empty(){
            anyhow::bail!("请正确配置《腾讯》翻译引擎！！！")
        }
        let text = self.get_selected()?;
        self.source_lang = self.simple_language_check(&text)?;
        self.target_lang = self.get_target_lang()?;

        let mut map: BTreeMap<&str,String> = BTreeMap::new();
        map.insert("Action","TextTranslate".to_string());
        map.insert("Version","2018-03-21".to_string());
        map.insert("Region",self.engine.region.to_string());
        map.insert("Timestamp",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
        map.insert("Nonce",rand::random::<u16>().to_string());
        map.insert("SecretId",self.engine.appid.to_string());
        map.insert("SignatureMethod","HmacSHA1".to_string());
        map.insert("SourceText",text);
        map.insert("Source",self.source_lang.to_string());
        map.insert("Target",self.target_lang.to_string());
        map.insert("ProjectId","0".to_string());

        let param_str = map.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join("&");
        let source_sign_string = format!("GET{}/?{}", self.engine.url, param_str);
        let sign = self.hmac_sha1(source_sign_string)?;
        debug!("source sign: {}", sign);

        let url_param_str = map.iter().map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");
        self.request_url = format!("https://{}?{}&Signature={}",self.engine.url,url_param_str,sign);
        Ok(self)
    }

    fn get_request_url(&self) -> anyhow::Result<String> {
        Ok(self.request_url.clone())
    }

    fn result_json_handler(&mut self, result_json: String) -> anyhow::Result<&Self> {
        debug!("腾讯翻译返回结果：{}",result_json);
        self.result_json = serde_json::from_str(&result_json)?;;
        Ok(self)
    }

    fn translate_result(self) -> anyhow::Result<TranslateResult> {
        let result = self.result_json.unwrap();
        debug!("{:?}",&result);
        if result.response.error.is_none() {
            Ok(TranslateResult{
                success: true,
                msg: "".to_string(),
                target_text: result.response.target_text.unwrap(),
                source_text: "".to_string(),
                target_lang: self.target_lang,
                source_lang: self.source_lang,
                video_url: "".to_string(),
                symbol: "".to_string(),
            })
        } else {
            let error = result.response.error.unwrap();
            anyhow::bail!("翻译错误：{} => {}",error.message,error.code);
        }
    }

    fn get_hmac_key(&self) -> String {
        self.engine.engine_key.to_string()
    }
}