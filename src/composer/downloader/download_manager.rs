// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct DownloadManager {
}

impl DownloadManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setPreferSource(&self, preferSource: bool) -> Self {
        todo!()
    }

    pub fn setPreferDist(&self, preferDist: bool) -> Self {
        todo!()
    }

    pub fn setPreferences(&self, preferences: Vec<serde_json::Value>) -> Self {
        todo!()
    }

    pub fn setDownloader(&self, r#type: String, downloader: Box<dyn crate::composer::downloader::downloader_interface::DownloaderInterface>) -> Self {
        todo!()
    }

    pub fn getDownloader(&self, r#type: String) -> Box<dyn crate::composer::downloader::downloader_interface::DownloaderInterface> {
        todo!()
    }

    pub fn getDownloaderForPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<Box<dyn crate::composer::downloader::downloader_interface::DownloaderInterface>> {
        todo!()
    }

    pub fn getDownloaderType(&self, downloader: Box<dyn crate::composer::downloader::downloader_interface::DownloaderInterface>) -> String {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn prepare(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String) -> serde_json::Value {
        todo!()
    }

    pub fn update(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String) -> serde_json::Value {
        todo!()
    }

    pub fn remove(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String) -> serde_json::Value {
        todo!()
    }

    pub fn cleanup(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, targetDir: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value {
        todo!()
    }

    pub(crate) fn resolvePackageInstallPreference(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> String {
        todo!()
    }

    fn getAvailableSources(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn normalizeTargetDir(&self, dir: String) -> String {
        todo!()
    }

}

