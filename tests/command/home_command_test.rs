// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct HomeCommandTest {
}

impl HomeCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testHomeCommandWithShowFlag(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String, urls: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn useCaseProvider() -> serde_json::Value {
        todo!()
    }

}

