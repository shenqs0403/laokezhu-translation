use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use tauri_plugin_log::log::debug;
use tauri_plugin_log::log::kv::ToKey;
use uuid::Uuid;
use crate::common::read_selected_text;
use crate::dao::Engine;
use crate::translate_v1::{TranslateResult, Translator};

#[derive(Deserialize, Debug,Default,Serialize,Clone)]
struct BaiduTransDataResult {
    #[serde(default="String::new")]
    src: String,
    #[serde(default="String::new")]
    dst: String,
}
#[derive(Deserialize, Debug,Default,Serialize,Clone)]
struct BaiduResult {
    #[serde(default="String::new")]
    from: String,
    #[serde(default="String::new")]
    to: String,
    #[serde(default = "Vec::new")]
    trans_result: Vec<BaiduTransDataResult>,
    #[serde(default="String::new")]
    error_code: String,
    #[serde(default="String::new")]
    error_msg: String,
}

impl BaiduResult {
    fn error(code: String,msg: String) -> BaiduResult {
        BaiduResult{
            from: "".to_string(),
            to: "".to_string(),
            trans_result: vec![],
            error_code: code,
            error_msg: msg,
        }
    }
}

pub struct BaiduTranslator {
    engine: Engine,
    source_lang: String,
    target_lang: String,
    request_url: String,
    result_json: Option<BaiduResult>,
}

impl BaiduTranslator {
    pub fn new(engine: Engine, source_lang: String,target_lang: String) -> Self {
        BaiduTranslator {
            engine,
            source_lang,
            target_lang,
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
            return Ok("cht".to_string());
        }
        Ok(string.split("-").next().unwrap().to_string())
    }
}

impl Translator for BaiduTranslator {
    fn prepare_data(&mut self) -> anyhow::Result<&Self> {
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() {
            anyhow::bail!("请正确配置《百度》翻译引擎！！！")
        }
        debug!("{:?}", self.engine);
        let text = self.get_selected()?;
        let salt: u16 = rand::random();
        let sign = hex::encode(md5::compute(format!("{}{}{}{}", self.engine.appid, text, salt, self.engine.engine_key)).0);
        self.request_url = format!("{}?from=auto&to={}&appid={}&salt={}&sign={}&q={}",self.engine.url,self.default_target_lang()?,self.engine.appid,salt,sign,urlencoding::encode(&text));
        debug!("百度引擎请求url:{}", self.request_url);
        Ok(self)
    }

    fn get_request_url(&self) -> anyhow::Result<String> {
        Ok(self.request_url.clone())
    }

    fn result_json_handler(&mut self, result_json: String) -> anyhow::Result<&Self> {
        let result = serde_json::from_str::<BaiduResult>(result_json.as_str())?;
        if !result.error_code.is_empty() {
            anyhow::bail!("翻译失败：{}",result.error_msg)
        }
        self.result_json = Some(result);
        Ok(self)
    }

    fn translate_result(self) -> anyhow::Result<TranslateResult> {
        let result = self.result_json.unwrap();
        Ok(TranslateResult {
            success: true,
            msg: "操作成功".to_string(),
            target_text: result.trans_result.iter().map(|x| x.dst.to_string()).collect::<Vec<String>>().join("  "),
            source_text: result.trans_result.iter().map(|x| x.src.to_string()).collect::<Vec<String>>().join("  "),
            target_lang: result.to,
            source_lang: result.from,
            video_url: "".to_string(),
            symbol: "".to_string(),
        })
    }
}
