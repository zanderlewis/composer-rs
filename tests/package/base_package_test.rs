// Namespace: Composer\Test\Package

#[derive(Debug, Clone, Default)]
pub struct BasePackageTest {
}

impl BasePackageTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testSetSameRepository(&self) {
        todo!()
    }

    pub fn testSetAnotherRepository(&self) {
        todo!()
    }

    pub fn testFormatVersionForDevPackage(&self, sourceReference: String, truncate: bool, expected: String) {
        todo!()
    }

    pub fn provideFormattedVersions() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testPackageNamesToRegexp(&self, packageNames: Vec<serde_json::Value>, wrap: serde_json::Value, expectedRegexp: String) {
        todo!()
    }

    pub fn dataPackageNamesToRegexp() -> Vec<serde_json::Value> {
        todo!()
    }

}

