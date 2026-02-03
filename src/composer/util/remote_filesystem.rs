// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct RemoteFilesystem {
}

impl RemoteFilesystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copy(&self, originUrl: String, fileUrl: String, fileName: String, progress: bool, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getContents(&self, originUrl: String, fileUrl: String, progress: bool, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getOptions(&self) {
        todo!()
    }

    pub fn setOptions(&self, options: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn isTlsDisabled(&self) {
        todo!()
    }

    pub fn getLastHeaders(&self) {
        todo!()
    }

    pub fn findStatusCode(headers: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn findStatusMessage(&self, headers: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn get(&self, originUrl: String, fileUrl: String, additionalOptions: Vec<serde_json::Value>, fileName: Option<String>, progress: bool) {
        todo!()
    }

    pub(crate) fn getRemoteContents(&self, originUrl: String, fileUrl: String, context: serde_json::Value, responseHeaders: Option<Vec<serde_json::Value>>, maxFileSize: Option<i64>) {
        todo!()
    }

    pub(crate) fn callbackGet(&self, notificationCode: i64, severity: i64, message: Option<String>, messageCode: i64, bytesTransferred: i64, bytesMax: i64) {
        todo!()
    }

    pub(crate) fn promptAuthAndRetry(&self, httpStatus: serde_json::Value, reason: Option<String>, headers: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn getOptionsForUrl(&self, originUrl: String, additionalOptions: Vec<serde_json::Value>) {
        todo!()
    }

    fn handleRedirect(&self, responseHeaders: Vec<serde_json::Value>, additionalOptions: Vec<serde_json::Value>, result: serde_json::Value) {
        todo!()
    }

    fn decodeResult(&self, result: serde_json::Value, responseHeaders: Vec<serde_json::Value>) -> Option<String> {
        todo!()
    }

    fn normalizeResult(&self, result: serde_json::Value) -> Option<String> {
        todo!()
    }

}

