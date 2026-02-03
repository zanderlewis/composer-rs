// Namespace: Composer\Util\Http

#[derive(Debug, Clone, Default)]
pub struct Response {
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getStatusCode(&self) -> i64 {
        todo!()
    }

    pub fn getStatusMessage(&self) -> Option<String> {
        todo!()
    }

    pub fn getHeaders(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getHeader(&self, name: String) -> Option<String> {
        todo!()
    }

    pub fn getBody(&self) -> Option<String> {
        todo!()
    }

    pub fn decodeJson(&self) {
        todo!()
    }

    pub fn collect(&self) {
        todo!()
    }

    pub fn findHeaderValue(headers: Vec<serde_json::Value>, name: String) -> Option<String> {
        todo!()
    }

}

