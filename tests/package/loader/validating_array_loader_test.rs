// Namespace: Composer\Test\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct ValidatingArrayLoaderTest {
}

impl ValidatingArrayLoaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testLoadSuccess(&self, config: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn successProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testLoadFailureThrowsException(&self, config: Vec<serde_json::Value>, expectedErrors: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testLoadWarnings(&self, config: Vec<serde_json::Value>, expectedWarnings: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testLoadSkipsWarningDataWhenIgnoringErrors(&self, config: Vec<serde_json::Value>, expectedWarnings: Vec<serde_json::Value>, mustCheck: bool, expectedArray: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    pub fn errorProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn warningProvider() -> Vec<serde_json::Value> {
        todo!()
    }

}

