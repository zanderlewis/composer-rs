// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct ShowCommandTest {
}

impl ShowCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testShow(&self, command: Vec<serde_json::Value>, expected: String, requires: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn provideShow() -> serde_json::Value {
        todo!()
    }

    pub fn testOutdatedFiltersAccordingToPlatformReqsAndWarns(&self) {
        todo!()
    }

    pub fn testOutdatedFiltersAccordingToPlatformReqsWithoutWarningForHigherVersions(&self) {
        todo!()
    }

    pub fn testShowDirectWithNameDoesNotShowTransientDependencies(&self) {
        todo!()
    }

    pub fn testShowDirectWithNameOnlyShowsDirectDependents(&self) {
        todo!()
    }

    pub fn testShowPlatformOnlyShowsPlatformPackages(&self) {
        todo!()
    }

    pub fn testShowPlatformWorksWithoutComposerJson(&self) {
        todo!()
    }

    pub fn testOutdatedWithZeroMajor(&self) {
        todo!()
    }

    pub fn testShowAllShowsAllSections(&self) {
        todo!()
    }

    pub fn testLockedRequiresValidLockFile(&self) {
        todo!()
    }

    pub fn testLockedShowsAllLocked(&self) {
        todo!()
    }

    pub fn testInvalidOptionCombinations(&self) {
        todo!()
    }

    pub fn testIgnoredOptionCombinations(&self) {
        todo!()
    }

    pub fn testSelfAndNameOnly(&self) {
        todo!()
    }

    pub fn testSelfAndPackageCombination(&self) {
        todo!()
    }

    pub fn testSelf(&self) {
        todo!()
    }

    pub fn testNotInstalledError(&self) {
        todo!()
    }

    pub fn testNoDevOption(&self) {
        todo!()
    }

    pub fn testPackageFilter(&self) {
        todo!()
    }

    pub fn testNotExistingPackage(&self, package: String, options: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn provideNotExistingPackage(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testNotExistingPackageWithWorkingDir(&self) {
        todo!()
    }

    pub fn testSpecificPackageAndTree(&self, callable: Box<dyn Fn()>, options: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn providePackageAndTree(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testNameOnlyPrintsNoTrailingWhitespace(&self) {
        todo!()
    }

}

