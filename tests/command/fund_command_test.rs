// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct FundCommandTest {
}

impl FundCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testFundCommand(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, funding: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn useCaseProvider() -> serde_json::Value {
        todo!()
    }

}

