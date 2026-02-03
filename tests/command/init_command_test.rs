// Namespace: Composer\Test\Command

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct InitCommandTest {
}

impl InitCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub fn testParseValidAuthorString(&self, name: String, email: Option<String>, input: String) {
        todo!()
    }

    pub fn validAuthorStringProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testParseEmptyAuthorString(&self) {
        todo!()
    }

    pub fn testParseAuthorStringWithInvalidEmail(&self) {
        todo!()
    }

    pub fn testNamespaceFromValidPackageName(&self) {
        todo!()
    }

    pub fn testNamespaceFromInvalidPackageName(&self) {
        todo!()
    }

    pub fn testNamespaceFromMissingPackageName(&self) {
        todo!()
    }

    fn callParseAuthorString(&self, command: crate::composer::command::init_command::InitCommand, string: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRunCommand(&self, expected: Vec<serde_json::Value>, arguments: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn runDataProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRunCommandInvalid(&self, exception: Option<String>, message: Option<String>, arguments: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn runInvalidDataProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRunGuessNameFromDirSanitizesDir(&self) {
        todo!()
    }

    pub fn testInteractiveRun(&self) {
        todo!()
    }

    pub fn testFormatAuthors(&self) {
        todo!()
    }

    pub fn testGetGitConfig(&self) {
        todo!()
    }

    pub fn testAddVendorIgnore(&self) {
        todo!()
    }

    pub fn testHasVendorIgnore(&self) {
        todo!()
    }

}

#[derive(Debug, Clone, Default)]
pub struct DummyInitCommand {
}

impl DummyInitCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn formatAuthors(&self, author: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getGitConfig(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn addVendorIgnore(&self, ignoreFile: String, vendor: String) {
        todo!()
    }

    pub fn hasVendorIgnore(&self, ignoreFile: String, vendor: String) -> bool {
        todo!()
    }

}

