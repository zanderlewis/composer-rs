// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct PoolBuilderTest {
}

impl PoolBuilderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testPoolBuilder(&self, file: String, message: String, expect: Vec<serde_json::Value>, expectOptimized: Vec<serde_json::Value>, root: Vec<serde_json::Value>, requestData: Vec<serde_json::Value>, packageRepos: Vec<serde_json::Value>, fixed: Vec<serde_json::Value>) {
        todo!()
    }

    fn getPackageResultSet(&self, pool: crate::composer::dependency_resolver::pool::Pool, packageIds: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getIntegrationTests() -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn readTestFile(file: String, fixturesDir: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

