use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tauri_plugin_log::log::debug;
use tauri_plugin_log::log::kv::ToKey;
use uuid::Uuid;
use crate::dao::Engine;
use crate::translate_v1::{TranslateResult, Translator};

#[derive(Serialize,Default,Debug,Deserialize,Clone)]
struct YoudaoResult {
    #[serde(rename="errorCode")]
    error_code: String,
    query: String,
    translation: Vec<String>,
    #[serde(rename="tSpeakUrl")]
    t_speak_url: String,
    l: String,
}

pub struct YoudaoTranslator {
    engine:Engine,
    target_lang: String,
    source_lang: String,
    request_url: String,
    result_json: Option<YoudaoResult>,
}
impl YoudaoTranslator {
    pub fn new(engine: Engine,source_lang: String,target_lang: String) -> YoudaoTranslator {
        YoudaoTranslator {
            engine,
            target_lang,
            source_lang,
            request_url: "".to_string(),
            result_json: None,
        }
    }

    fn default_target_lang(&self) -> anyhow::Result<String> {
        if !self.target_lang.is_empty() {
            return Ok(self.target_lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if ["zh-HK","zh-SG","zh-TW"].contains(&string.as_str()) {
            return Ok("zh-CHT".to_string());
        }
        if "zh-CN".eq(&string) {
            return Ok("zh-CNS".to_string());
        }
        Ok(string.split("-").next().unwrap().to_string())
    }

    fn get_input(&self,text: &String) -> anyhow::Result<String> {
        if text.len() <= 20 {
            return Ok(text.to_string());
        }
        let s = format!("{}{}{}",&text[0..10],&text.len(),&text[&text.len() - 10..]);
        Ok(s)
    }
}

impl Translator for YoudaoTranslator {
    fn prepare_data(&mut self) -> anyhow::Result<&Self> {
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() {
            anyhow::bail!("请正确配置《有道》翻译引擎！！！");
        }
        let text = self.get_selected()?;
        let salt = Uuid::new_v4().to_string();
        let input = self.get_input(&text)?;
        let curtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        let sign = hex::encode(sha2::Sha256::digest(format!("{}{}{}{}{}",self.engine.appid,input,salt,curtime,self.engine.engine_key)));
        let url = format!("{}?q={}&from=auto&to={}&appKey={}&salt={}&sign={}&signType=v3&curtime={}",self.engine.url,text,self.default_target_lang()?,self.engine.appid,salt,sign,curtime);
        debug!("有道翻译请求url:{}",url);
        self.request_url = url;
        Ok(self)
    }

    fn get_request_url(&self) -> anyhow::Result<String> {
        Ok(self.request_url.clone())
    }

    fn result_json_handler(&mut self, result_json: String) -> anyhow::Result<&Self> {
        self.result_json = Some(serde_json::from_str::<YoudaoResult>(&result_json)?);
        Ok(self)
    }

    fn translate_result(self) -> anyhow::Result<TranslateResult> {
        let result = self.result_json.unwrap();
        let vec = result.l.split("2").collect::<Vec<&str>>();
        Ok(TranslateResult {
            success: true,
            msg: if result.error_code != "0" { format!("查询错误:{}", result.error_code) } else { "操作成功".to_string() },
            target_text: result.translation.join("   "),
            source_text: "".to_string(),
            target_lang: vec[1].to_string(),
            source_lang: vec[0].to_string(),
            video_url: result.t_speak_url,
            symbol: "".to_string(),
        })
    }
}