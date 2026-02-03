// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct ZipDownloader {
}

impl ZipDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>, output: bool) -> serde_json::Value {
        todo!()
    }

    fn extractWithSystemUnzip(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: String, path: String) -> serde_json::Value {
        todo!()
    }

    fn extractWithZipArchive(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: String, path: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn extract(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: String, path: String) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn getErrorMessage(&self, retval: i64, file: String) -> String {
        todo!()
    }

}

