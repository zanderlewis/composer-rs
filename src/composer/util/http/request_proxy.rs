// Namespace: Composer\Util\Http

#[derive(Debug, Clone, Default)]
pub struct RequestProxy {
}

impl RequestProxy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> crate::composer::util::http::request_proxy::RequestProxy {
        todo!()
    }

    pub fn noProxy() -> crate::composer::util::http::request_proxy::RequestProxy {
        todo!()
    }

    pub fn getContextOptions(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn getCurlOptions(&self, sslOptions: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getStatus(&self, format: Option<String>) -> String {
        todo!()
    }

    pub fn isExcludedByNoProxy(&self) -> bool {
        todo!()
    }

    pub fn isSecure(&self) -> bool {
        todo!()
    }

    pub fn supportsSecureProxy(&self) -> bool {
        todo!()
    }

}

