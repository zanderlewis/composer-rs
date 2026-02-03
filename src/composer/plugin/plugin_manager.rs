// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct PluginManager {
}

impl PluginManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setRunningInGlobalDir(&self, runningInGlobalDir: bool) {
        todo!()
    }

    pub fn loadInstalledPlugins(&self) {
        todo!()
    }

    pub fn deactivateInstalledPlugins(&self) {
        todo!()
    }

    pub fn getPlugins(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRegisteredPlugins(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getGlobalComposer(&self) -> Option<crate::composer::partial_composer::PartialComposer> {
        todo!()
    }

    pub fn registerPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, failOnMissingClasses: bool, isGlobalPlugin: bool) {
        todo!()
    }

    pub fn deactivatePackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub fn uninstallPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) {
        todo!()
    }

    pub(crate) fn getPluginApiVersion(&self) -> String {
        todo!()
    }

    pub fn addPlugin(&self, plugin: Box<dyn crate::composer::plugin::plugin_interface::PluginInterface>, isGlobalPlugin: bool, sourcePackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn removePlugin(&self, plugin: Box<dyn crate::composer::plugin::plugin_interface::PluginInterface>) {
        todo!()
    }

    pub fn uninstallPlugin(&self, plugin: Box<dyn crate::composer::plugin::plugin_interface::PluginInterface>) {
        todo!()
    }

    fn loadRepository(&self, repo: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>, isGlobalRepo: bool, rootPackage: Option<Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>>) {
        todo!()
    }

    fn deactivateRepository(&self, repo: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>, isGlobalRepo: bool) {
        todo!()
    }

    fn collectDependencies(&self, installedRepo: crate::composer::repository::installed_repository::InstalledRepository, collected: Vec<serde_json::Value>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getInstallPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, global: bool) -> Option<String> {
        todo!()
    }

    pub(crate) fn getCapabilityImplementationClassName(&self, plugin: Box<dyn crate::composer::plugin::plugin_interface::PluginInterface>, capability: String) -> Option<String> {
        todo!()
    }

    pub fn getPluginCapability(&self, plugin: Box<dyn crate::composer::plugin::plugin_interface::PluginInterface>, capabilityClassName: serde_json::Value, ctorArgs: Vec<serde_json::Value>) -> Option<crate::tests::composer::test::plugin::mock::capability::Capability> {
        todo!()
    }

    pub fn getPluginCapabilities(&self, capabilityClassName: serde_json::Value, ctorArgs: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn parseAllowedPlugins(&self, allowPluginsConfig: serde_json::Value, locker: Option<crate::composer::package::locker::Locker>) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn arePluginsDisabled(&self, r#type: serde_json::Value) {
        todo!()
    }

    pub fn disablePlugins(&self) {
        todo!()
    }

    pub fn isPluginAllowed(&self, package: String, isGlobalPlugin: bool, optional: bool, prompt: bool) -> bool {
        todo!()
    }

}

