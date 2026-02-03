// Namespace: Composer\Command

pub trait PackageDiscoveryTrait {
    fn getRepos(&self) -> crate::composer::repository::composite_repository::CompositeRepository { todo!() }
    fn getRepositorySet(&self, input: serde_json::Value, minimumStability: Option<String>) -> crate::composer::repository::repository_set::RepositorySet { todo!() }
    fn getMinimumStability(&self, input: serde_json::Value) -> String { todo!() }
    fn determineRequirements(&self, input: serde_json::Value, output: serde_json::Value, requires: Vec<serde_json::Value>, platformRepo: Option<crate::composer::repository::platform_repository::PlatformRepository>, preferredStability: String, useBestVersionConstraint: bool, fixed: bool) -> Vec<serde_json::Value> { todo!() }
    fn findBestVersionAndNameForPackage(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, input: serde_json::Value, name: String, platformRepo: Option<crate::composer::repository::platform_repository::PlatformRepository>, preferredStability: String, fixed: bool) -> Vec<serde_json::Value> { todo!() }
    fn findSimilar(&self, package: String) -> Vec<serde_json::Value> { todo!() }
    fn getPlatformExceptionDetails(&self, candidate: Box<dyn crate::composer::package::package_interface::PackageInterface>, platformRepo: Option<crate::composer::repository::platform_repository::PlatformRepository>) -> String { todo!() }
}

