// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct RepositoryUtils {
}

impl RepositoryUtils {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filterRequiredPackages(packages: Vec<serde_json::Value>, requirer: Box<dyn crate::composer::package::package_interface::PackageInterface>, includeRequireDev: bool, bucket: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn flattenRepositories(repo: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>, unwrapFilterRepos: bool) -> Vec<serde_json::Value> {
        todo!()
    }

}

