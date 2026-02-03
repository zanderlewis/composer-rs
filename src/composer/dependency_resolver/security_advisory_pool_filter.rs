// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct SecurityAdvisoryPoolFilter {
}

impl SecurityAdvisoryPoolFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(&self, pool: crate::composer::dependency_resolver::pool::Pool, repositories: Vec<serde_json::Value>, request: crate::composer::dependency_resolver::request::Request) -> crate::composer::dependency_resolver::pool::Pool {
        todo!()
    }

    fn getMatchingAdvisories(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, advisoryMap: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

