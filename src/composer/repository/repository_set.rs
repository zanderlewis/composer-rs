// Namespace: Composer\Repository

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct RepositorySet {
}

impl RepositorySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allowInstalledRepositories(&self, allow: bool) {
        todo!()
    }

    pub fn getRootRequires(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getTemporaryConstraints(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn addRepository(&self, repo: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>) {
        todo!()
    }

    pub fn findPackages(&self, name: String, constraint: Option<serde_json::Value>, flags: i64) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getSecurityAdvisories(&self, packageNames: Vec<serde_json::Value>, allowPartialAdvisories: bool, ignoreUnreachable: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getMatchingSecurityAdvisories(&self, packages: Vec<serde_json::Value>, allowPartialAdvisories: bool, ignoreUnreachable: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getSecurityAdvisoriesForConstraints(&self, packageConstraintMap: Vec<serde_json::Value>, allowPartialAdvisories: bool, ignoreUnreachable: bool, unreachableRepos: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getProviders(&self, packageName: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isPackageAcceptable(&self, names: Vec<serde_json::Value>, stability: String) -> bool {
        todo!()
    }

    pub fn createPool(&self, request: crate::composer::dependency_resolver::request::Request, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, eventDispatcher: Option<crate::composer::event_dispatcher::event_dispatcher::EventDispatcher>, poolOptimizer: Option<crate::composer::dependency_resolver::pool_optimizer::PoolOptimizer>, ignoredTypes: Vec<serde_json::Value>, allowedTypes: Option<Vec<serde_json::Value>>, securityAdvisoryPoolFilter: Option<crate::composer::dependency_resolver::security_advisory_pool_filter::SecurityAdvisoryPoolFilter>) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    pub fn createPoolWithAllPackages(&self) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    pub fn createPoolForPackage(&self, packageName: String, lockedRepo: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    pub fn createPoolForPackages(&self, packageNames: Vec<serde_json::Value>, lockedRepo: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    fn getRootAliasesPerPackage(aliases: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

