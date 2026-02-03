// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct ArchiveDownloader {
}

impl ArchiveDownloader {
    pub fn new() -> Self {
        Self::default()
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

    pub(crate) fn getInstallOperationAppendix(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> String {
        todo!()
    }

    pub(crate) fn extract(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: String, path: String) -> serde_json::Value {
        todo!()
    }

}

