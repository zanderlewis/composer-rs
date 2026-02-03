// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct BaseDependencyCommandTest {
}

impl BaseDependencyCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testExceptionWhenNoRequiredParameters(&self, command: String, parameters: Vec<serde_json::Value>, expectedExceptionMessage: String) {
        todo!()
    }

    pub fn noParametersCaseProvider() -> serde_json::Value {
        todo!()
    }

    pub fn testExceptionWhenRunningLockedWithoutLockFile(&self, command: String, parameters: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testExceptionWhenItCouldNotFoundThePackage(&self, command: String, parameters: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testExceptionWhenPackageWasNotFoundInProject(&self, command: String, parameters: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testWarningWhenDependenciesAreNotInstalled(&self, command: String, parameters: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn caseProvider() -> serde_json::Value {
        todo!()
    }

    pub fn testWhyCommandOutputs(&self, parameters: Vec<serde_json::Value>, expectedOutput: String, expectedStatusCode: i64) {
        todo!()
    }

    pub fn caseWhyProvider() -> serde_json::Value {
        todo!()
    }

    pub fn testWhyNotCommandOutputs(&self, parameters: Vec<serde_json::Value>, expectedOutput: String, expectedStatusCode: i64) {
        todo!()
    }

    pub fn caseWhyNotProvider(&self) -> serde_json::Value {
        todo!()
    }

}

