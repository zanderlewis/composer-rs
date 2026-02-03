// Namespace: Composer\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct BaseExcludeFilter {
}

impl BaseExcludeFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(&self, relativePath: String, exclude: bool) -> bool {
        todo!()
    }

    pub(crate) fn parseLines(&self, lines: Vec<serde_json::Value>, lineParser: Box<dyn Fn()>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn generatePatterns(&self, rules: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn generatePattern(&self, rule: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

