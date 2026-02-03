// Namespace: Composer\Test\Command

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ValidateCommandTest {
}

impl ValidateCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testValidate(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn testValidateOnFileIssues(&self) {
        todo!()
    }

    pub fn testWithComposerLock(&self) {
        todo!()
    }

    pub fn testUnaccessibleFile(&self) {
        todo!()
    }

    pub fn provideValidateTests() -> serde_json::Value {
        todo!()
    }

}

