// Namespace: Composer\Test\Json

#[derive(Debug, Clone, Default)]
pub struct ComposerSchemaTest {
}

impl ComposerSchemaTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testNamePattern(&self) {
        todo!()
    }

    pub fn versionProvider(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testVersionPattern(&self, version: String, isValid: bool) {
        todo!()
    }

    pub fn testOptionalAbandonedProperty(&self) {
        todo!()
    }

    pub fn testRequireTypes(&self) {
        todo!()
    }

    pub fn testMinimumStabilityValues(&self) {
        todo!()
    }

    pub fn assertAmbiguousRepositoryNotPossible(&self) {
        todo!()
    }

    fn check(&self, json: String) {
        todo!()
    }

}

