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
}

impl Translator for AliyunTranslator {
    async fn start_http(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}