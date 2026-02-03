// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct PluginInstaller {
}

impl PluginInstaller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn supports(&self, packageType: String) {
        todo!()
    }

    pub fn disablePlugins(&self) {
        todo!()
    }

    pub fn prepare(&self, r#type: serde_json::Value, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
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

    fn rollbackInstall(&self, e: serde_json::Value, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub(crate) fn getPluginManager(&self) -> crate::composer::plugin::plugin_manager::PluginManager {
        todo!()
    }

}

