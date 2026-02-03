// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct CheckPlatformReqsCommandTest {
}

impl CheckPlatformReqsCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testPlatformReqsAreSatisfied(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String, lock: bool) {
        todo!()
    }

    pub fn testExceptionThrownIfNoLockfileFound(&self) {
        todo!()
    }

    pub fn caseProvider() -> serde_json::Value {
        todo!()
    }

    pub fn testFailedPlatformRequirement(&self) {
        todo!()
    }

}

