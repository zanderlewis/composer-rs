// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct ZipDownloaderTest {
}

impl ZipDownloaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn setPrivateProperty(&self, name: String, value: serde_json::Value, obj: serde_json::Value) {
        todo!()
    }

    pub fn testErrorMessages(&self) {
        todo!()
    }

    pub fn testZipArchiveOnlyFailed(&self) {
        todo!()
    }

    pub fn testZipArchiveExtractOnlyFailed(&self) {
        todo!()
    }

    pub fn testZipArchiveOnlyGood(&self) {
        todo!()
    }

    pub fn testSystemUnzipOnlyFailed(&self) {
        todo!()
    }

    pub fn testSystemUnzipOnlyGood(&self) {
        todo!()
    }

    pub fn testNonWindowsFallbackGood(&self) {
        todo!()
    }

    pub fn testNonWindowsFallbackFailed(&self) {
        todo!()
    }

    fn wait(&self, promise: serde_json::Value) {
        todo!()
    }

}

#[derive(Debug, Clone, Default)]
pub struct MockedZipDownloader {
}

impl MockedZipDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: serde_json::Value, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: serde_json::Value, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn extract(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: serde_json::Value, path: serde_json::Value) -> serde_json::Value {
        todo!()
    }

}

