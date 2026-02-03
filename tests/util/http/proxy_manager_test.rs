// Namespace: Composer\Test\Util\Http

#[derive(Debug, Clone, Default)]
pub struct ProxyManagerTest {
}

impl ProxyManagerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testInstantiation(&self) {
        todo!()
    }

    pub fn testGetProxyForRequestThrowsOnBadProxyUrl(&self) {
        todo!()
    }

    pub fn testLowercaseOverridesUppercase(&self, server: Vec<serde_json::Value>, url: String, expectedUrl: String) {
        todo!()
    }

    pub fn dataCaseOverrides() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testCGIProxyIsOnlyUsedWhenNoHttpProxy(&self, server: Vec<serde_json::Value>, expectedUrl: String) {
        todo!()
    }

    pub fn dataCGIProxy() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testNoHttpProxyDoesNotUseHttpsProxy(&self) {
        todo!()
    }

    pub fn testNoHttpsProxyDoesNotUseHttpProxy(&self) {
        todo!()
    }

    pub fn testGetProxyForRequest(&self, server: Vec<serde_json::Value>, url: String, options: Option<Vec<serde_json::Value>>, status: String, excluded: bool) {
        todo!()
    }

    pub fn dataRequest() -> Vec<serde_json::Value> {
        todo!()
    }

}

