// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct MaxFileSizeExceededException {
}

impl MaxFileSizeExceededException {
    pub fn new() -> Self {
        Self::default()
    }

}

