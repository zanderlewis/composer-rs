// Namespace: Composer\Test

#[derive(Debug, Clone, Default)]
pub struct InstalledVersionsTest {
}

impl InstalledVersionsTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUpBeforeClass() {
        todo!()
    }

    pub fn tearDownAfterClass() {
        todo!()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub fn testGetInstalledPackages(&self) {
        todo!()
    }

    pub fn testIsInstalled(&self, expected: bool, name: String, includeDevRequirements: bool) {
        todo!()
    }

    pub fn isInstalledProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testSatisfies(&self, expected: bool, name: String, constraint: String) {
        todo!()
    }

    pub fn satisfiesProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetVersionRanges(&self, expected: String, name: String) {
        todo!()
    }

    pub fn getVersionRangesProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetVersion(&self, expected: Option<String>, name: String) {
        todo!()
    }

    pub fn getVersionProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetPrettyVersion(&self, expected: Option<String>, name: String) {
        todo!()
    }

    pub fn getPrettyVersionProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetVersionOutOfBounds(&self) {
        todo!()
    }

    pub fn testGetRootPackage(&self) {
        todo!()
    }

    pub fn testGetRawData(&self) {
        todo!()
    }

    pub fn testGetReference(&self, expected: Option<String>, name: String) {
        todo!()
    }

    pub fn getReferenceProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetInstalledPackagesByType(&self) {
        todo!()
    }

    pub fn testGetInstallPath(&self) {
        todo!()
    }

    pub fn testWithClassLoaderLoaded(&self) {
        todo!()
    }

}

