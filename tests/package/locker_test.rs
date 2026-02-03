// Namespace: Composer\Test\Package

#[derive(Debug, Clone, Default)]
pub struct LockerTest {
}

impl LockerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testIsLocked(&self) {
        todo!()
    }

    pub fn testGetNotLockedPackages(&self) {
        todo!()
    }

    pub fn testGetLockedPackages(&self) {
        todo!()
    }

    pub fn testSetLockData(&self) {
        todo!()
    }

    pub fn testLockBadPackages(&self) {
        todo!()
    }

    pub fn testIsFresh(&self) {
        todo!()
    }

    pub fn testIsFreshFalse(&self) {
        todo!()
    }

    pub fn testIsFreshWithContentHash(&self) {
        todo!()
    }

    pub fn testIsFreshWithContentHashAndNoHash(&self) {
        todo!()
    }

    pub fn testIsFreshFalseWithContentHash(&self) {
        todo!()
    }

    fn createJsonFileMock(&self) {
        todo!()
    }

    fn createInstallationManagerMock(&self) {
        todo!()
    }

    fn createPackageMock(&self) {
        todo!()
    }

    fn getJsonContent(&self, customData: Vec<serde_json::Value>) -> String {
        todo!()
    }

}

