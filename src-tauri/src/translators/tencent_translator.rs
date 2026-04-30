use crate::dao::Engine;
use crate::translators::Translator;

pub struct TencentTranslator {
    engine: Engine,
    lang: String,
}
impl TencentTranslator {
    pub fn new(engine: Engine,lang: String) -> Self {
        TencentTranslator { engine, lang }
    }
}
impl Translator for TencentTranslator {
    async fn start_http(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}