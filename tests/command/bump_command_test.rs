// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct BumpCommandTest {
}

impl BumpCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testBump(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: Vec<serde_json::Value>, lock: bool, exitCode: i64) {
        todo!()
    }

    pub fn testBumpFailsOnNonExistingComposerFile(&self) {
        todo!()
    }

    pub fn testBumpFailsOnWriteErrorToComposerFile(&self) {
        todo!()
    }

    pub fn provideTests() -> serde_json::Value {
        todo!()
    }

}

