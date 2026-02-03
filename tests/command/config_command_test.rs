// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct ConfigCommandTest {
}

impl ConfigCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testConfigUpdates(&self, before: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn provideConfigUpdates() -> serde_json::Value {
        todo!()
    }

    pub fn testConfigReads(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn provideConfigReads() -> serde_json::Value {
        todo!()
    }

    pub fn testConfigThrowsForInvalidArgCombination(&self) {
        todo!()
    }

    pub fn testConfigThrowsForInvalidSeverity(&self) {
        todo!()
    }

    pub fn testConfigThrowsWhenMergingArrayWithObject(&self) {
        todo!()
    }

}

