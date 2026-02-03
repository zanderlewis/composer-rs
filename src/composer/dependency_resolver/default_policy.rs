// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct DefaultPolicy {
}

impl DefaultPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn versionCompare(&self, a: Box<dyn crate::composer::package::package_interface::PackageInterface>, b: Box<dyn crate::composer::package::package_interface::PackageInterface>, operator: String) -> bool {
        todo!()
    }

    pub fn selectPreferredPackages(&self, pool: crate::composer::dependency_resolver::pool::Pool, literals: Vec<serde_json::Value>, requiredPackage: Option<String>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn groupLiteralsByName(&self, pool: crate::composer::dependency_resolver::pool::Pool, literals: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn compareByPriority(&self, pool: crate::composer::dependency_resolver::pool::Pool, a: crate::composer::package::base_package::BasePackage, b: crate::composer::package::base_package::BasePackage, requiredPackage: Option<String>, ignoreReplace: bool) -> i64 {
        todo!()
    }

    pub(crate) fn replaces(&self, source: crate::composer::package::base_package::BasePackage, target: crate::composer::package::base_package::BasePackage) -> bool {
        todo!()
    }

    pub(crate) fn pruneToBestVersion(&self, pool: crate::composer::dependency_resolver::pool::Pool, literals: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn pruneRemoteAliases(&self, pool: crate::composer::dependency_resolver::pool::Pool, literals: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

