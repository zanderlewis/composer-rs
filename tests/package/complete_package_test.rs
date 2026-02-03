// Namespace: Composer\Test\Package

#[derive(Debug, Clone, Default)]
pub struct CompletePackageTest {
}

impl CompletePackageTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn providerVersioningSchemes() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testPackageHasExpectedNamingSemantics(&self, name: String, version: String) {
        todo!()
    }

    pub fn testPackageHasExpectedVersioningSemantics(&self, name: String, version: String) {
        todo!()
    }

    pub fn testPackageHasExpectedMarshallingSemantics(&self, name: String, version: String) {
        todo!()
    }

    pub fn testGetTargetDir(&self) {
        todo!()
    }

}

