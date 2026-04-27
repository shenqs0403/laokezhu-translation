use tauri_plugin_log::log::debug;
use uuid::Uuid;
use crate::common::read_selected_text;
use crate::dao::Engine;
use crate::translators::Translator;
use tauri_plugin_http::reqwest;

pub struct BaiduTranslator {
    engine: Engine,
    lang: String,
}
impl BaiduTranslator {
    pub fn new(engine: Engine, lang: String) -> Self {
        BaiduTranslator {engine, lang}
    }

    fn get_lang(&self) -> anyhow::Result<String> {
        if !self.lang.is_empty() { 
            return Ok(self.lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if ["zh_HK","zh_SG","zh_TW"].contains(&string.as_str()) {
            return Ok("cht".to_string());
        }
        Ok(string.split("_").next().unwrap().to_string())
    }
}
impl Translator for BaiduTranslator {
    async fn start_http(&mut self) -> anyhow::Result<String> {
        // 校验参数
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() {
            anyhow::bail!("配置不完整，请在配置界面完善《百度》翻译引擎配置");
        }

        // 读取选中的文本内容
        let text = read_selected_text()?;
        if text.trim().is_empty() {
            return Ok(String::new());
        }
        let salt = Uuid::new_v4().to_string();
        let digest = hex::encode(md5::compute(format!("{}{}{}{}", self.engine.appid, text, salt, self.engine.engine_key)).0);
        let url = format!("{}?q={}&from=auto&to={}&appid={}&salt={}&sign={}", self.engine.url, text, self.get_lang()?, self.engine.appid, salt, digest);
        debug!("百度翻译url:{}",url);
        let string = reqwest::get(url).await?.text().await?;
        debug!("百度翻译返回结果：{}",string);
        Ok(string)
    }
}