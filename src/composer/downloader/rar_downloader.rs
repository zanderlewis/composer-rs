// Namespace: Composer\Downloader

#[derive(Debug, Clone, Default)]
pub struct RarDownloader {
}

impl RarDownloader {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn extract(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, file: String, path: String) -> serde_json::Value {
        todo!()
    }

}

