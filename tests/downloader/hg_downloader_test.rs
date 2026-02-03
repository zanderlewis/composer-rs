// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct HgDownloaderTest {
}

impl HgDownloaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub(crate) fn getDownloaderMock(&self, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, config: Option<crate::composer::config::Config>, executor: Option<crate::tests::composer::test::mock::process_executor_mock::ProcessExecutorMock>, filesystem: Option<crate::tests::composer::test::fixtures::functional::installed_versions2::vendor::symfony::filesystem::filesystem::Filesystem>) -> crate::composer::downloader::hg_downloader::HgDownloader {
        todo!()
    }

    pub fn testDownloadForPackageWithoutSourceReference(&self) {
        todo!()
    }

    pub fn testDownload(&self) {
        todo!()
    }

    pub fn testUpdateforPackageWithoutSourceReference(&self) {
        todo!()
    }

    pub fn testUpdate(&self) {
        todo!()
    }

    pub fn testRemove(&self) {
        todo!()
    }

    pub fn testGetInstallationSource(&self) {
        todo!()
    }

}

