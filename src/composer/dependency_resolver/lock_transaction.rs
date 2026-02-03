// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct LockTransaction {
}

impl LockTransaction {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setResultPackages(&self, pool: crate::composer::dependency_resolver::pool::Pool, decisions: crate::composer::dependency_resolver::decisions::Decisions) {
        todo!()
    }

    pub fn setNonDevPackages(&self, extractionResult: crate::composer::dependency_resolver::lock_transaction::LockTransaction) {
        todo!()
    }

    pub fn getNewLockPackages(&self, devMode: bool, updateMirrors: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    fn updateMirrorAndUrls(&self, package: crate::composer::package::base_package::BasePackage) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn getAliases(&self, aliases: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

