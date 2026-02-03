// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Loop {
}

impl Loop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getHttpDownloader(&self) -> crate::composer::util::http_downloader::HttpDownloader {
        todo!()
    }

    pub fn getProcessExecutor(&self) -> Option<crate::composer::util::process_executor::ProcessExecutor> {
        todo!()
    }

    pub fn wait(&self, promises: Vec<serde_json::Value>, progress: Option<serde_json::Value>) {
        todo!()
    }

    pub fn abortJobs(&self) {
        todo!()
    }

}

