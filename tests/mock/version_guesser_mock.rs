// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct VersionGuesserMock {
}

impl VersionGuesserMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn guessVersion(&self, packageConfig: Vec<serde_json::Value>, path: serde_json::Value) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

}

