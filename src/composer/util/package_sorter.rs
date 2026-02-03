// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct PackageSorter {
}

impl PackageSorter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getMostCurrentVersion(packages: Vec<serde_json::Value>) -> Option<Box<dyn crate::composer::package::package_interface::PackageInterface>> {
        todo!()
    }

    pub fn sortPackagesAlphabetically(packages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn sortPackages(packages: Vec<serde_json::Value>, weights: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

