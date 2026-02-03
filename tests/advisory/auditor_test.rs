// Namespace: Composer\Test\Advisory

#[derive(Debug, Clone, Default)]
pub struct AuditorTest {
}

impl AuditorTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auditProvider() {
        todo!()
    }

    pub fn testAudit(&self, data: Vec<serde_json::Value>, expected: i64, output: String) {
        todo!()
    }

    pub fn ignoredIdsProvider(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testAuditWithIgnore(&self, packages: serde_json::Value, ignoredIds: serde_json::Value, exitCode: serde_json::Value, expectedOutput: serde_json::Value) {
        todo!()
    }

    pub fn ignoreSeverityProvider(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testAuditWithIgnoreUnreachable(&self) {
        todo!()
    }

    pub fn testAuditWithIgnoreSeverity(&self, packages: serde_json::Value, ignoredSeverities: serde_json::Value, exitCode: serde_json::Value, expectedOutput: serde_json::Value) {
        todo!()
    }

    fn getRepoSet(&self) -> crate::composer::repository::repository_set::RepositorySet {
        todo!()
    }

    pub fn getMockAdvisories() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testNeedsCompleteAdvisoryLoad(&self, advisories: Vec<serde_json::Value>, ignoreList: Vec<serde_json::Value>, expected: bool) {
        todo!()
    }

    pub fn needsCompleteLoadProvider() -> Vec<serde_json::Value> {
        todo!()
    }

}

