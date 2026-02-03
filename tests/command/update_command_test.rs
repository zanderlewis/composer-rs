// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct UpdateCommandTest {
}

impl UpdateCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testUpdate(&self, composerJson: Vec<serde_json::Value>, command: Vec<serde_json::Value>, expected: String, createLock: bool) {
        todo!()
    }

    pub fn provideUpdates() -> serde_json::Value {
        todo!()
    }

    pub fn testUpdateWithPatchOnly(&self) {
        todo!()
    }

    pub fn testInteractiveModeThrowsIfNoPackageToUpdate(&self) {
        todo!()
    }

    pub fn testInteractiveModeThrowsIfNoPackageEntered(&self) {
        todo!()
    }

    pub fn testInteractiveTmp(&self, packageNames: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn provideInteractiveUpdates(&self) -> serde_json::Value {
        todo!()
    }

    pub fn testNoSecurityBlockingAllowsInsecurePackages(&self) {
        todo!()
    }

    pub fn testBumpAfterUpdateWithoutLockfile(&self) {
        todo!()
    }

}

