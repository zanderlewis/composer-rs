// Namespace: Composer\Test\Plugin

#[derive(Debug, Clone, Default)]
pub struct PluginInstallerTest {
}

impl PluginInstallerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testInstallNewPlugin(&self) {
        todo!()
    }

    pub fn testInstallPluginWithRootPackageHavingFilesAutoload(&self) {
        todo!()
    }

    pub fn testInstallMultiplePlugins(&self) {
        todo!()
    }

    pub fn testUpgradeWithNewClassName(&self) {
        todo!()
    }

    pub fn testUninstall(&self) {
        todo!()
    }

    pub fn testUpgradeWithSameClassName(&self) {
        todo!()
    }

    pub fn testRegisterPluginOnlyOneTime(&self) {
        todo!()
    }

    fn setPluginApiVersionWithPlugins(&self, newPluginApiVersion: String, plugins: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testStarPluginVersionWorksWithAnyAPIVersion(&self) {
        todo!()
    }

    pub fn testPluginConstraintWorksOnlyWithCertainAPIVersion(&self) {
        todo!()
    }

    pub fn testPluginRangeConstraintsWorkOnlyWithCertainAPIVersion(&self) {
        todo!()
    }

    pub fn testCommandProviderCapability(&self) {
        todo!()
    }

    pub fn testIncapablePluginIsCorrectlyDetected(&self) {
        todo!()
    }

    pub fn testCapabilityImplementsComposerPluginApiClassAndIsConstructedWithArgs(&self) {
        todo!()
    }

    pub fn invalidImplementationClassNames() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testQueryingWithInvalidCapabilityClassNameThrows(&self, invalidImplementationClassNames: serde_json::Value, expect: String) {
        todo!()
    }

    pub fn testQueryingNonProvidedCapabilityReturnsNullSafely(&self) {
        todo!()
    }

    pub fn nonExistingOrInvalidImplementationClassTypes() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testQueryingWithNonExistingOrWrongCapabilityClassTypesThrows(&self, wrongImplementationClassTypes: String) {
        todo!()
    }

}

