// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct PoolBuilder {
}

impl PoolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setIgnoredTypes(&self, types: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn setAllowedTypes(&self, types: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    pub fn buildPool(&self, repositories: Vec<serde_json::Value>, request: crate::composer::dependency_resolver::request::Request) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    fn markPackageNameForLoading(&self, request: crate::composer::dependency_resolver::request::Request, name: String, constraint: serde_json::Value) {
        todo!()
    }

    fn loadPackagesMarkedForLoading(&self, request: crate::composer::dependency_resolver::request::Request, repositories: Vec<serde_json::Value>) {
        todo!()
    }

    fn loadPackage(&self, request: crate::composer::dependency_resolver::request::Request, repositories: Vec<serde_json::Value>, package: crate::composer::package::base_package::BasePackage, propagateUpdate: bool) {
        todo!()
    }

    fn isRootRequire(&self, request: crate::composer::dependency_resolver::request::Request, name: String) -> bool {
        todo!()
    }

    fn getSkippedRootRequires(&self, request: crate::composer::dependency_resolver::request::Request, name: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn isUpdateAllowed(&self, package: crate::composer::package::base_package::BasePackage) -> bool {
        todo!()
    }

    fn warnAboutNonMatchingUpdateAllowList(&self, request: crate::composer::dependency_resolver::request::Request) {
        todo!()
    }

    fn unlockPackage(&self, request: crate::composer::dependency_resolver::request::Request, repositories: Vec<serde_json::Value>, name: String) {
        todo!()
    }

    fn markPackageNameForLoadingIfRequired(&self, request: crate::composer::dependency_resolver::request::Request, name: String) {
        todo!()
    }

    fn removeLoadedPackage(&self, request: crate::composer::dependency_resolver::request::Request, repositories: Vec<serde_json::Value>, package: crate::composer::package::base_package::BasePackage, index: i64) {
        todo!()
    }

    fn runOptimizer(&self, request: crate::composer::dependency_resolver::request::Request, pool: crate::composer::dependency_resolver::pool::Pool) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    fn runSecurityAdvisoryFilter(&self, pool: crate::composer::dependency_resolver::pool::Pool, repositories: Vec<serde_json::Value>, request: crate::composer::dependency_resolver::request::Request) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

}

