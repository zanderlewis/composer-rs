// Namespace: Composer\Console

#[derive(Debug, Clone, Default)]
pub struct HtmlOutputFormatter {
}

impl HtmlOutputFormatter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format(&self, message: Option<String>) -> Option<String> {
        todo!()
    }

    fn formatHtml(&self, matches: Vec<serde_json::Value>) -> String {
        todo!()
    }

}

