// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct HttpDownloaderMock {
}

impl HttpDownloaderMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expects(&self, expectations: Vec<serde_json::Value>, strict: bool, defaultHandler: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn assertComplete(&self) {
        todo!()
    }

    pub fn get(&self, fileUrl: serde_json::Value, options: serde_json::Value) -> crate::composer::util::http::response::Response {
        todo!()
    }

    fn respond(&self, url: String, status: i64, headers: Vec<serde_json::Value>, body: String) -> crate::composer::util::http::response::Response {
        todo!()
    }

}

