// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct FileDownloader {
}

impl FileDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInstallationSource(&self) -> String {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn prepare(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn cleanup(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, output: bool) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn getDistPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, component: i64) -> String {
        todo!()
    }

    pub(crate) fn clearLastCacheWrite(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub(crate) fn addCleanupPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) {
        todo!()
    }

    pub(crate) fn removeCleanupPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) {
        todo!()
    }

    pub fn update(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value {
        todo!()
    }

    pub fn remove(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, output: bool) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn getFileName(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> String {
        todo!()
    }

    pub(crate) fn getInstallOperationAppendix(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> String {
        todo!()
    }

    pub(crate) fn processUrl(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, url: String) -> String {
        todo!()
    }

    pub fn getLocalChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

}

