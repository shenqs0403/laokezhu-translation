use std::collections::{BTreeMap, HashMap};
use tauri_plugin_log::log::debug;
use crate::common::read_selected_text;
use crate::dao::Engine;
use crate::translators::Translator;

pub struct AliyunTranslator {
    engine: Engine,
    lang: String,
}

impl AliyunTranslator {
    pub fn new(engine: Engine,lang: String) -> AliyunTranslator {
        AliyunTranslator{ engine, lang }
    }

    /// 先弄个简陋的文本语言识别，只有中英文之分
    fn get_text_lang(&self,text: &String) -> String {
        if text.len() != text.chars().count() {
            "zh".to_string()
        } else {
            "en".to_string()
        }
    }

    fn get_lang(&self) -> anyhow::Result<String> {
        if !self.lang.is_empty() {
            return Ok(self.lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if ["zh-HK","zh-SG","zh-TW"].contains(&string.as_str()) {
            return Ok("zh-tw".to_string());
        }
        Ok(string.split("-").next().unwrap().to_string())
    }

}

impl Translator for AliyunTranslator {
    async fn start_http(&mut self) -> anyhow::Result<String> {
        let mut map: BTreeMap<&str, String> = BTreeMap::new();
        map.insert("Action","TranslateGeneral".to_string());
        map.insert("Version","2018-10-12".to_string());
        map.insert("Format","JSON".to_string());
        map.insert("AccessKeyId","LTAI5t7TG5EXNF5F5DC8aroj".to_string());
        map.insert("SignatureNonce",rand::random::<u32>().to_string());
        map.insert("Timestamp",chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        map.insert("SignatureMethod","HMAC-SHA1".to_string());
        map.insert("SignatureVersion","1.0".to_string());
        map.insert("FormatType","text".to_string());
        map.insert("Scene","general".to_string());

        let text = read_selected_text()?;
        map.insert("SourceLanguage",self.get_text_lang(&text));
        map.insert("SourceText",text);
        map.insert("TargetLanguage",self.get_lang()?);

        let param_str = map.iter().map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<String>>().join("&");

        let string_to_sign = format!("GET&{}&{}",urlencoding::encode("/"),urlencoding::encode(&param_str));
        debug!("本地签名字符串：{}",string_to_sign);
        let sign = self.hmac_sha1(string_to_sign)?;
        debug!("签名字符串：{}",sign);
        let url = format!("{}?{}&Signature={}",self.engine.url,param_str,urlencoding::encode(&sign));
        debug!("阿里云翻译url:{}",url);

        let string = tauri_plugin_http::reqwest::get(url)
            .await?
            .text()
            .await?;
        // 加一个语言返回
        let mut result_map: HashMap<String,serde_json::Value> = serde_json::from_str(&string)?;
        result_map.insert("source_lang".to_string(),serde_json::Value::String(map.get("SourceLanguage").unwrap().to_string()));
        result_map.insert("target_lang".to_string(),serde_json::Value::String(map.get("TargetLanguage").unwrap().to_string()));
        let string = serde_json::to_string(&result_map)?;
        debug!("请求返回：{}",string);
        Ok(string)
    }

    fn get_key(&self) -> String {
        format!("{}&",self.engine.engine_key)
    }
}