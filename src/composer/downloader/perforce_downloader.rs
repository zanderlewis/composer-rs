// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct PerforceDownloader {
}

impl PerforceDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn doDownload(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn doInstall(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String) -> serde_json::Value {
        todo!()
    }

    fn getLabelFromSourceReference(&self, r#ref: String) -> Option<String> {
        todo!()
    }

    pub fn initPerforce(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String) {
        todo!()
    }

    fn getRepoConfig(&self, repository: crate::composer::repository::vcs_repository::VcsRepository) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn doUpdate(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String) -> serde_json::Value {
        todo!()
    }

    pub fn getLocalChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn getCommitLogs(&self, fromReference: String, toReference: String, path: String) -> String {
        todo!()
    }

    pub fn setPerforce(&self, perforce: crate::composer::util::perforce::Perforce) {
        todo!()
    }

    pub(crate) fn hasMetadataRepository(&self, path: String) -> bool {
        todo!()
    }

}

