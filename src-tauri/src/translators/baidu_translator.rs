use std::ops::Index;
use crate::common::read_selected_text;
use crate::dao::Engine;
use crate::translators::Translator;

pub struct BaiduTranslator {
    engine: Engine,
}
impl BaiduTranslator {
    pub fn new(engine: Engine) -> Self {
        BaiduTranslator {engine}
    }

    fn get_lang(&self) -> anyhow::Result<String> {
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

        // format!("{}?q={}&from=auto&to={}&appid={}&salt={}&sign={}",self.engine.url,text,)
        Ok(text)
    }
}