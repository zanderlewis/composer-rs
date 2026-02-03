// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct PreFileDownloadEvent {
}

impl PreFileDownloadEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getHttpDownloader(&self) -> crate::composer::util::http_downloader::HttpDownloader {
        todo!()
    }

    pub fn getProcessedUrl(&self) -> String {
        todo!()
    }

    pub fn setProcessedUrl(&self, processedUrl: String) {
        todo!()
    }

    pub fn getCustomCacheKey(&self) -> Option<String> {
        todo!()
    }

    pub fn setCustomCacheKey(&self, customCacheKey: Option<String>) {
        todo!()
    }

    pub fn getType(&self) -> String {
        todo!()
    }

    pub fn getContext(&self) {
        todo!()
    }

    pub fn getTransportOptions(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setTransportOptions(&self, options: Vec<serde_json::Value>) {
        todo!()
    }

}

