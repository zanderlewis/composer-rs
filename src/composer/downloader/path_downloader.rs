// Namespace: Composer\Downloader

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct PathDownloader {
}

impl PathDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn install(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn remove(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String, output: bool) -> serde_json::Value {
        todo!()
    }

    pub fn getVcsReference(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String> {
        todo!()
    }

    pub(crate) fn getInstallOperationAppendix(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> String {
        todo!()
    }

    fn computeAllowedStrategies(&self, transportOptions: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn safeJunctions(&self) -> bool {
        todo!()
    }

}

