// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct FilesystemRepository {
}

impl FilesystemRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getDevMode(&self) {
        todo!()
    }

    pub(crate) fn initialize(&self) {
        todo!()
    }

    pub fn reload(&self) {
        todo!()
    }

    pub fn write(&self, devMode: bool, installationManager: crate::composer::installer::installation_manager::InstallationManager) {
        todo!()
    }

    pub fn safelyLoadInstalledVersions(path: String) -> bool {
        todo!()
    }

    fn dumpToPhpCode(&self, array: Vec<serde_json::Value>, level: i64) -> String {
        todo!()
    }

    fn generateInstalledVersions(&self, installationManager: crate::composer::installer::installation_manager::InstallationManager, installPaths: Vec<serde_json::Value>, devMode: bool, repoDir: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn dumpInstalledPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, installPaths: Vec<serde_json::Value>, repoDir: String, devPackages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn dumpRootPackage(&self, package: Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>, installPaths: Vec<serde_json::Value>, devMode: bool, repoDir: String, devPackages: Vec<serde_json::Value>) {
        todo!()
    }

}

