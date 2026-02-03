// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct PackageSorterTest {
}

impl PackageSorterTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testSortingDoesNothingWithNoDependencies(&self) {
        todo!()
    }

    pub fn sortingOrdersDependenciesHigherThanPackageDataProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testSortingOrdersDependenciesHigherThanPackage(&self, packages: Vec<serde_json::Value>, expectedOrderedList: Vec<serde_json::Value>, weights: Vec<serde_json::Value>) {
        todo!()
    }

    fn createPackage(name: String, requires: Vec<serde_json::Value>) -> crate::composer::package::package::Package {
        todo!()
    }

}

