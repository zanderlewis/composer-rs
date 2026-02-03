// Namespace: Composer\Package\Version

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct VersionParser {
}

impl VersionParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parseConstraints(&self, constraints: serde_json::Value) -> serde_json::Value {
        todo!()
    }

    pub fn parseNameVersionPairs(&self, pairs: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isUpgrade(normalizedFrom: String, normalizedTo: String) -> bool {
        todo!()
    }

}

