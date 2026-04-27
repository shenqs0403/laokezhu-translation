use std::time::SystemTime;
use sha2::Digest;
use tauri_plugin_log::log::debug;
use uuid::Uuid;
use crate::common::read_selected_text;
use crate::dao::Engine;
use crate::translators::Translator;

pub struct YoudaoTranslator {
    engine: Engine,
    lang: String,
}
impl YoudaoTranslator{
    pub fn new(engine: Engine, lang: String) -> YoudaoTranslator {
        YoudaoTranslator{engine, lang}
    }
    fn get_lang(&self) -> anyhow::Result<String> {
        if !self.lang.is_empty() {
            return Ok(self.lang.clone());
        }
        let string = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if ["zh-HK","zh-SG","zh-TW"].contains(&string.as_str()) {
            return Ok("cht".to_string());
        }
        Ok(string.split("-").next().unwrap().to_string())
    }

    fn get_input(&self,text: String) -> anyhow::Result<String> {
        if text.len() <= 20 {
            return Ok(text.to_string());
        }
        let s = format!("{}{}{}",&text[0..10],&text.len(),&text[&text.len() - 10..]);
        Ok(s)
    }
}
impl Translator for YoudaoTranslator{
    async fn start_http(&mut self) -> anyhow::Result<String> {
        if self.engine.url.is_empty() || self.engine.appid.is_empty() || self.engine.engine_key.is_empty() {
            anyhow::bail!("配置不完整，请在配置界面完善《有道》翻译引擎配置")
        }
        let text = read_selected_text()?;
        debug!("读取选中的文本:{}",text);
        let salt = Uuid::new_v4().to_string();
        let input = self.get_input(text.clone())?;
        debug!("input 字符串：{}",input);
        let curtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        let sign = hex::encode(sha2::Sha256::digest(format!("{}{}{}{}{}",self.engine.appid,input,salt,curtime,self.engine.engine_key)));

        let url = format!("{}?q={}&from=auto&to={}&appKey={}&salt={}&sign={}&signType=v3&curtime={}",self.engine.url,text,self.get_lang()?,self.engine.appid,salt,sign,curtime);
        debug!("有道翻译url:{}",url);
        let res_str = tauri_plugin_http::reqwest::get(url).await?.text().await?;
        debug!("有道翻译结果：{}",res_str);
        Ok(res_str)
    }
}