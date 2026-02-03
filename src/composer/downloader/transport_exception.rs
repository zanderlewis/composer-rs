// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct TransportException {
}

impl TransportException {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setHeaders(&self, headers: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getHeaders(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    pub fn setResponse(&self, response: Option<String>) {
        todo!()
    }

    pub fn getResponse(&self) -> Option<String> {
        todo!()
    }

    pub fn setStatusCode(&self, statusCode: serde_json::Value) {
        todo!()
    }

    pub fn getStatusCode(&self) -> Option<i64> {
        todo!()
    }

    pub fn getResponseInfo(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setResponseInfo(&self, responseInfo: Vec<serde_json::Value>) {
        todo!()
    }

}

