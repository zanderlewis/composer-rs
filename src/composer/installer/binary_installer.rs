// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct BinaryInstaller {
}

impl BinaryInstaller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn installBinaries(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, installPath: String, warnOnOverwrite: bool) {
        todo!()
    }

    pub fn removeBinaries(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn determineBinaryCaller(bin: String) -> String {
        todo!()
    }

    pub(crate) fn getBinaries(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn installFullBinaries(&self, binPath: String, link: String, bin: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub(crate) fn installUnixyProxyBinaries(&self, binPath: String, link: String) {
        todo!()
    }

    pub(crate) fn initializeBinDir(&self) {
        todo!()
    }

    pub(crate) fn generateWindowsProxyCode(&self, bin: String, link: String) -> String {
        todo!()
    }

    pub(crate) fn generateUnixyProxyCode(&self, bin: String, link: String) -> String {
        todo!()
    }

}

