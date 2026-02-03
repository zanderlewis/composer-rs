// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct InstalledRepository {
}

impl InstalledRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn findPackagesWithReplacersAndProviders(&self, name: String, constraint: serde_json::Value) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getDependents(&self, needle: serde_json::Value, constraint: Option<serde_json::Value>, invert: bool, recurse: bool, packagesFound: Option<Vec<serde_json::Value>>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRepoName(&self) -> String {
        todo!()
    }

    pub fn addRepository(&self, repository: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>) {
        todo!()
    }

}

