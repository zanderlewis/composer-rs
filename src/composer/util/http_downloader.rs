// Namespace: Composer\Util

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct HttpDownloader {
}

impl HttpDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, url: String, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn add(&self, url: String, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn copy(&self, url: String, to: String, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn addCopy(&self, url: String, to: String, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getOptions(&self) {
        todo!()
    }

    pub fn setOptions(&self, options: Vec<serde_json::Value>) {
        todo!()
    }

    fn addJob(&self, request: Vec<serde_json::Value>, sync: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    fn startJob(&self, id: i64) {
        todo!()
    }

    fn markJobDone(&self) {
        todo!()
    }

    pub fn wait(&self, index: Option<i64>) {
        todo!()
    }

    pub fn enableAsync(&self) {
        todo!()
    }

    pub fn countActiveJobs(&self, index: Option<i64>) -> i64 {
        todo!()
    }

    fn getResponse(&self, index: i64) -> crate::composer::util::http::response::Response {
        todo!()
    }

    pub fn outputWarnings(io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, url: String, data: serde_json::Value) {
        todo!()
    }

    pub fn getExceptionHints(e: serde_json::Value) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

    fn canUseCurl(&self, job: Vec<serde_json::Value>) -> bool {
        todo!()
    }

    pub fn isCurlEnabled() -> bool {
        todo!()
    }

}

