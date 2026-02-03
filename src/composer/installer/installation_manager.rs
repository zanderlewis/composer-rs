// Namespace: Composer\Installer

#[derive(Debug, Clone, Default)]
pub struct InstallationManager {
}

impl InstallationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&self) {
        todo!()
    }

    pub fn addInstaller(&self, installer: Box<dyn crate::composer::installer::installer_interface::InstallerInterface>) {
        todo!()
    }

    pub fn removeInstaller(&self, installer: Box<dyn crate::composer::installer::installer_interface::InstallerInterface>) {
        todo!()
    }

    pub fn disablePlugins(&self) {
        todo!()
    }

    pub fn getInstaller(&self, r#type: String) -> Box<dyn crate::composer::installer::installer_interface::InstallerInterface> {
        todo!()
    }

    pub fn isPackageInstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> bool {
        todo!()
    }

    pub fn ensureBinariesPresence(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn execute(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operations: Vec<serde_json::Value>, devMode: bool, runScripts: bool, downloadOnly: bool) {
        todo!()
    }

    fn downloadAndExecuteBatch(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operations: Vec<serde_json::Value>, cleanupPromises: Vec<serde_json::Value>, devMode: bool, runScripts: bool, downloadOnly: bool, allOperations: Vec<serde_json::Value>) {
        todo!()
    }

    fn executeBatch(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, operations: Vec<serde_json::Value>, cleanupPromises: Vec<serde_json::Value>, devMode: bool, runScripts: bool, allOperations: Vec<serde_json::Value>) {
        todo!()
    }

    fn waitOnPromises(&self, promises: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<serde_json::Value> {
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

    pub fn getInstallPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<String> {
        todo!()
    }

    pub fn setOutputProgress(&self, outputProgress: bool) {
        todo!()
    }

    pub fn notifyInstalls(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    fn markForNotification(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    fn runCleanup(&self, cleanupPromises: Vec<serde_json::Value>) {
        todo!()
    }

}

