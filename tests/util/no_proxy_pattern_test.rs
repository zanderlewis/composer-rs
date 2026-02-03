// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct NoProxyPatternTest {
}

impl NoProxyPatternTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testHostName(&self, noproxy: String, url: String, expected: bool) {
        todo!()
    }

    pub fn dataHostName() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testIpAddress(&self, noproxy: String, url: String, expected: bool) {
        todo!()
    }

    pub fn dataIpAddress() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testIpRange(&self, noproxy: String, url: String, expected: bool) {
        todo!()
    }

    pub fn dataIpRange() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testPort(&self, noproxy: String, url: String, expected: bool) {
        todo!()
    }

    pub fn dataPort() -> Vec<serde_json::Value> {
        todo!()
    }

    fn getUrl(&self, url: String) -> String {
        todo!()
    }

}

