// Namespace: Composer\Test\Platform

#[derive(Debug, Clone, Default)]
pub struct VersionTest {
}

impl VersionTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn provideOpenSslVersions() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testParseOpensslVersions(&self, input: String, parsedVersion: String, fipsExpected: bool, normalizedVersion: Option<String>) {
        todo!()
    }

    pub fn provideLibJpegVersions() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testParseLibjpegVersion(&self, input: String, parsedVersion: String) {
        todo!()
    }

    pub fn provideZoneinfoVersions() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testParseZoneinfoVersion(&self, input: String, parsedVersion: String) {
        todo!()
    }

}

