// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct StatusCommandTest {
}

impl StatusCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testNoLocalChanges(&self) {
        todo!()
    }

    pub fn testLocallyModifiedPackages(&self, composerJson: Vec<serde_json::Value>, commandFlags: Vec<serde_json::Value>, packageData: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn locallyModifiedPackagesUseCaseProvider() -> serde_json::Value {
        todo!()
    }

}

