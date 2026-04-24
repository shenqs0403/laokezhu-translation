use crate::translators::Translator;

pub struct BaiduTranslator {}
impl BaiduTranslator {
    pub fn new() -> Self {
        BaiduTranslator {}
    }
}
impl Translator for BaiduTranslator {
    async fn start_http(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}