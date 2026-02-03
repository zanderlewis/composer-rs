// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct InstallCommandTest {
}

impl InstallCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testInstallCommandErrors(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn errorCaseProvider(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testInstallFromEmptyVendor(&self) {
        todo!()
    }

    pub fn testInstallFromEmptyVendorNoDev(&self) {
        todo!()
    }

    pub fn testInstallNewPackagesWithExistingPartialVendor(&self) {
        todo!()
    }

}

