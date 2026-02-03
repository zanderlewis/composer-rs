// Namespace: Composer\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct ArrayLoader {
}

impl ArrayLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self, config: Vec<serde_json::Value>, class: String) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn loadPackages(&self, versions: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn createObject(&self, config: Vec<serde_json::Value>, class: String) -> crate::composer::package::complete_package::CompletePackage {
        todo!()
    }

    fn configureObject(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, config: Vec<serde_json::Value>) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    fn configureCachedLinks(&self, linkCache: Vec<serde_json::Value>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, config: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn parseLinks(&self, source: String, sourceVersion: String, description: String, links: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn createLink(&self, source: String, sourceVersion: String, description: String, target: String, prettyConstraint: String) -> crate::composer::package::link::Link {
        todo!()
    }

    pub fn getBranchAlias(&self, config: Vec<serde_json::Value>) -> Option<String> {
        todo!()
    }

}

