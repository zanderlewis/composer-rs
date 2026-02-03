// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct VcsDownloader {
}

impl VcsDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInstallationSource(&self) -> String {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn prepare(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn cleanup(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value {
        todo!()
    }

    pub fn update(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value {
        todo!()
    }

    pub fn remove(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value {
        todo!()
    }

    pub fn getVcsReference(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn cleanChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, update: bool) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn reapplyChanges(&self, path: String) {
        todo!()
    }

    pub(crate) fn doDownload(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn doInstall(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn doUpdate(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, url: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn getCommitLogs(&self, fromReference: String, toReference: String, path: String) -> String {
        todo!()
    }

    pub(crate) fn hasMetadataRepository(&self, path: String) -> bool {
        todo!()
    }

    fn prepareUrls(&self, urls: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

