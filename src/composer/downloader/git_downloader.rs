// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct GitDownloader {
}

impl GitDownloader {
    pub fn new() -> Self {
        Self::default()
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

    pub fn getLocalChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

    pub fn getUnpushedChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn cleanChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, update: bool) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn reapplyChanges(&self, path: String) {
        todo!()
    }

    pub(crate) fn updateToCommit(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, reference: String, prettyVersion: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn updateOriginUrl(&self, path: String, url: String) {
        todo!()
    }

    pub(crate) fn setPushUrl(&self, path: String, url: String) {
        todo!()
    }

    pub(crate) fn getCommitLogs(&self, fromReference: String, toReference: String, path: String) -> String {
        todo!()
    }

    pub(crate) fn discardChanges(&self, path: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn stashChanges(&self, path: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn viewDiff(&self, path: String) {
        todo!()
    }

    pub(crate) fn normalizePath(&self, path: String) -> String {
        todo!()
    }

    pub(crate) fn hasMetadataRepository(&self, path: String) -> bool {
        todo!()
    }

    pub(crate) fn getShortHash(&self, reference: String) -> String {
        todo!()
    }

}

