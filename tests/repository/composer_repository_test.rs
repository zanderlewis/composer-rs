// Namespace: Composer\Test\Repository

#[derive(Debug, Clone, Default)]
pub struct ComposerRepositoryTest {
}

impl ComposerRepositoryTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testLoadData(&self, expected: Vec<serde_json::Value>, repoPackages: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn loadDataProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testWhatProvides(&self) {
        todo!()
    }

    pub fn testSearchWithType(&self) {
        todo!()
    }

    pub fn testSearchWithSpecialChars(&self) {
        todo!()
    }

    pub fn testSearchWithAbandonedPackages(&self) {
        todo!()
    }

    pub fn testCanonicalizeUrl(&self, expected: String, url: String, repositoryUrl: String) {
        todo!()
    }

    pub fn provideCanonicalizeUrlTestCases() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetProviderNamesWillReturnPartialPackageNames(&self) {
        todo!()
    }

    pub fn testGetSecurityAdvisoriesAssertRepositoryHttpOptionsAreUsed(&self) {
        todo!()
    }

    pub fn testGetSecurityAdvisoriesAssertRepositoryAdvisoriesIsZeroIndexedArrayWithConsecutiveKeys(&self) {
        todo!()
    }

    fn generateSecurityAdvisory(&self, packageName: String, cve: Option<String>, affectedVersions: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

