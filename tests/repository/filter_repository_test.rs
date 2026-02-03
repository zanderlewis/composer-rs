// Namespace: Composer\Test\Repository

#[derive(Debug, Clone, Default)]
pub struct FilterRepositoryTest {
}

impl FilterRepositoryTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub fn testRepoMatching(&self, expected: Vec<serde_json::Value>, config: serde_json::Value) {
        todo!()
    }

    pub fn provideRepoMatchingTestCases() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testBothFiltersDisallowed(&self) {
        todo!()
    }

    pub fn testSecurityAdvisoriesDisabledInChild(&self) {
        todo!()
    }

    pub fn testCanonicalDefaultTrue(&self) {
        todo!()
    }

    pub fn testNonCanonical(&self) {
        todo!()
    }

}

