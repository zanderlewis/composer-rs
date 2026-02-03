// Namespace: Composer\Downloader

pub trait DownloaderInterface {
    fn getInstallationSource(&self) -> String;
    fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value;
    fn prepare(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value;
    fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value;
    fn update(&self, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value;
    fn remove(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> serde_json::Value;
    fn cleanup(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> serde_json::Value;
}

