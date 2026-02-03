// Namespace: Composer\Package\Archiver

pub trait ArchiverInterface {
    fn archive(&self, sources: String, target: String, format: String, excludes: Vec<serde_json::Value>, ignoreFilters: bool) -> String;
    fn supports(&self, format: String, sourceType: Option<String>) -> bool;
}

