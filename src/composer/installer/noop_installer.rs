// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct NoopInstaller {
}

impl NoopInstaller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self, packageType: String) {
        todo!()
    }

    pub fn isInstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn prepare(&self, r#type: serde_json::Value, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn cleanup(&self, r#type: serde_json::Value, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn install(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn update(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn uninstall(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn getInstallPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

}

