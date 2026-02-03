// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct RemoveCommandTest {
}

impl RemoveCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testExceptionRunningWithNoRemovePackages(&self) {
        todo!()
    }

    pub fn testExceptionWhenRunningUnusedWithoutLockFile(&self) {
        todo!()
    }

    pub fn testWarningWhenRemovingNonExistentPackage(&self) {
        todo!()
    }

    pub fn testWarningWhenRemovingPackageFromWrongType(&self) {
        todo!()
    }

    pub fn testWarningWhenRemovingPackageWithDeprecatedDependenciesFlag(&self) {
        todo!()
    }

    pub fn testMessageOutputWhenNoUnusedPackagesToRemove(&self) {
        todo!()
    }

    pub fn testRemoveUnusedPackage(&self) {
        todo!()
    }

    pub fn testRemovePackageByName(&self) {
        todo!()
    }

    pub fn testRemovePackageByNameWithDryRun(&self) {
        todo!()
    }

    pub fn testRemoveAllowedPluginPackageWithNoOtherAllowedPlugins(&self) {
        todo!()
    }

    pub fn testRemoveAllowedPluginPackageWithOtherAllowedPlugins(&self) {
        todo!()
    }

    pub fn testRemovePackagesByVendor(&self) {
        todo!()
    }

    pub fn testRemovePackagesByVendorWithDryRun(&self) {
        todo!()
    }

    pub fn testWarningWhenRemovingPackagesByVendorFromWrongType(&self) {
        todo!()
    }

    pub fn testPackageStillPresentErrorWhenNoInstallFlagUsed(&self) {
        todo!()
    }

    pub fn testUpdateInheritedDependenciesFlagIsPassedToPostRemoveInstaller(&self, installFlagName: String, expectedComposerUpdateCommand: String) {
        todo!()
    }

    pub fn provideInheritedDependenciesUpdateFlag() -> serde_json::Value {
        todo!()
    }

}

