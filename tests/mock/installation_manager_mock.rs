// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct InstallationManagerMock {
}

impl InstallationManagerMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operations: Vec<serde_json::Value>, devMode: serde_json::Value, runScripts: serde_json::Value, downloadOnly: serde_json::Value) {
        todo!()
    }

    pub fn getInstallPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> String {
        todo!()
    }

    pub fn isPackageInstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> bool {
        todo!()
    }

    pub fn install(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operation: crate::composer::dependency_resolver::operation::install_operation::InstallOperation) -> Option<serde_json::Value> {
        todo!()
    }

    pub fn update(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operation: crate::composer::dependency_resolver::operation::update_operation::UpdateOperation) -> Option<serde_json::Value> {
        todo!()
    }

    pub fn uninstall(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operation: crate::composer::dependency_resolver::operation::uninstall_operation::UninstallOperation) -> Option<serde_json::Value> {
        todo!()
    }

    pub fn markAliasInstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operation: crate::composer::dependency_resolver::operation::mark_alias_installed_operation::MarkAliasInstalledOperation) {
        todo!()
    }

    pub fn markAliasUninstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operation: crate::composer::dependency_resolver::operation::mark_alias_uninstalled_operation::MarkAliasUninstalledOperation) {
        todo!()
    }

    pub fn getTrace(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getInstalledPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getUpdatedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getUninstalledPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn notifyInstalls(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    pub fn getInstalledPackagesByType(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

