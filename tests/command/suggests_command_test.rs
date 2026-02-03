// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct SuggestsCommandTest {
}

impl SuggestsCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testInstalledPackagesWithNoSuggestions(&self) {
        todo!()
    }

    pub fn testSuggest(&self, hasLockFile: bool, command: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn provideSuggest() -> serde_json::Value {
        todo!()
    }

    fn getPackageWithSuggestAndRequires(&self, name: String, version: String, suggests: Vec<serde_json::Value>, requires: Vec<serde_json::Value>, requireDevs: Vec<serde_json::Value>) -> crate::composer::package::complete_package::CompletePackage {
        todo!()
    }

}

