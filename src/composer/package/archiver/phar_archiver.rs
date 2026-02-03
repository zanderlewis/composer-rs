// Namespace: Composer\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct PharArchiver {
}

impl PharArchiver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn archive(&self, sources: String, target: String, format: String, excludes: Vec<serde_json::Value>, ignoreFilters: bool) -> String {
        todo!()
    }

    pub fn supports(&self, format: String, sourceType: Option<String>) -> bool {
        todo!()
    }

}

