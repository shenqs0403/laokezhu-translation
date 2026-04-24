use crate::translators::Translator;

pub struct YoudaoTranslator {}
impl YoudaoTranslator{
    pub fn new() -> YoudaoTranslator {
        YoudaoTranslator{}
    }
}
impl Translator for YoudaoTranslator{
    async fn start_http(&mut self) -> anyhow::Result<String> {
        todo!()
    }
}