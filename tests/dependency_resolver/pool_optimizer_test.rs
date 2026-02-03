// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct PoolOptimizerTest {
}

impl PoolOptimizerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testPoolOptimizer(&self, requestData: Vec<serde_json::Value>, packagesBefore: Vec<serde_json::Value>, expectedPackages: Vec<serde_json::Value>, message: String) {
        todo!()
    }

    pub fn provideIntegrationTests() -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn readTestFile(file: String, fixturesDir: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn reducePackagesInfoForComparison(&self, packages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn loadPackages(packagesData: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn loadPackage(packageData: Vec<serde_json::Value>) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

}

