// Namespace: Composer\Repository

pub trait RepositoryInterface {
    fn hasPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn findPackage(&self, name: String, constraint: serde_json::Value);
    fn findPackages(&self, name: String, constraint: serde_json::Value);
    fn getPackages(&self);
    fn loadPackages(&self, packageNameMap: Vec<serde_json::Value>, acceptableStabilities: Vec<serde_json::Value>, stabilityFlags: Vec<serde_json::Value>, alreadyLoaded: Vec<serde_json::Value>);
    fn search(&self, query: String, mode: i64, r#type: Option<String>);
    fn getProviders(&self, packageName: String);
    fn getRepoName(&self);
}

