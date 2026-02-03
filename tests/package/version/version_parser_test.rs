// Namespace: Composer\Test\Package\Version

#[derive(Debug, Clone, Default)]
pub struct VersionParserTest {
}

impl VersionParserTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testParseNameVersionPairs(&self, pairs: Vec<serde_json::Value>, result: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn provideParseNameVersionPairsData() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testIsUpgrade(&self, from: String, to: String, expected: bool) {
        todo!()
    }

    pub fn provideIsUpgradeTests() -> Vec<serde_json::Value> {
        todo!()
    }

}

