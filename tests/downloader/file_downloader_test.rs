// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct FileDownloaderTest {
}

impl FileDownloaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn getDownloader(&self, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, config: Option<crate::composer::config::Config>, eventDispatcher: Option<crate::composer::event_dispatcher::event_dispatcher::EventDispatcher>, cache: Option<crate::composer::cache::Cache>, httpDownloader: serde_json::Value, filesystem: Option<crate::tests::composer::test::fixtures::functional::installed_versions2::vendor::symfony::filesystem::filesystem::Filesystem>) -> crate::composer::downloader::file_downloader::FileDownloader {
        todo!()
    }

    pub fn testDownloadForPackageWithoutDistReference(&self) {
        todo!()
    }

    pub fn testDownloadToExistingFile(&self) {
        todo!()
    }

    pub fn testGetFileName(&self) {
        todo!()
    }

    pub fn testDownloadButFileIsUnsaved(&self) {
        todo!()
    }

    pub fn testDownloadWithCustomProcessedUrl(&self) {
        todo!()
    }

    pub fn testDownloadWithCustomCacheKey(&self) {
        todo!()
    }

    pub fn testCacheGarbageCollectionIsCalled(&self) {
        todo!()
    }

    pub fn testDownloadFileWithInvalidChecksum(&self) {
        todo!()
    }

    pub fn testDowngradeShowsAppropriateMessage(&self) {
        todo!()
    }

}

