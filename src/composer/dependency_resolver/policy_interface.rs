// Namespace: Composer\DependencyResolver

pub trait PolicyInterface {
    fn versionCompare(&self, a: Box<dyn crate::composer::package::package_interface::PackageInterface>, b: Box<dyn crate::composer::package::package_interface::PackageInterface>, operator: String) -> bool;
    fn selectPreferredPackages(&self, pool: crate::composer::dependency_resolver::pool::Pool, literals: Vec<serde_json::Value>, requiredPackage: Option<String>) -> Vec<serde_json::Value>;
}

