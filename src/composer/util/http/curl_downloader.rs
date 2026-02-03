// Namespace: Composer\Util\Http

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct CurlDownloader {
}

impl CurlDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn download(&self, resolve: Box<dyn Fn()>, reject: Box<dyn Fn()>, origin: String, url: String, options: Vec<serde_json::Value>, copyTo: Option<String>) -> i64 {
        todo!()
    }

    fn initDownload(&self, resolve: Box<dyn Fn()>, reject: Box<dyn Fn()>, origin: String, url: String, options: Vec<serde_json::Value>, copyTo: Option<String>, attributes: Vec<serde_json::Value>) -> i64 {
        todo!()
    }

    pub fn abortRequest(&self, id: i64) {
        todo!()
    }

    pub fn tick(&self) {
        todo!()
    }

    fn handleRedirect(&self, job: Vec<serde_json::Value>, response: crate::composer::util::http::response::Response) -> String {
        todo!()
    }

    fn isAuthenticatedRetryNeeded(&self, job: Vec<serde_json::Value>, response: crate::composer::util::http::response::Response) -> Vec<serde_json::Value> {
        todo!()
    }

    fn restartJob(&self, job: Vec<serde_json::Value>, url: String, attributes: Vec<serde_json::Value>) {
        todo!()
    }

    fn restartJobWithDelay(&self, job: Vec<serde_json::Value>, url: String, attributes: Vec<serde_json::Value>) {
        todo!()
    }

    fn failResponse(&self, job: Vec<serde_json::Value>, response: crate::composer::util::http::response::Response, errorMessage: String) -> crate::composer::downloader::transport_exception::TransportException {
        todo!()
    }

    fn rejectJob(&self, job: Vec<serde_json::Value>, e: serde_json::Value) {
        todo!()
    }

    fn checkCurlResult(&self, code: i64) {
        todo!()
    }

}

