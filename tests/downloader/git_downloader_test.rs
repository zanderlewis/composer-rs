// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct GitDownloaderTest {
}

impl GitDownloaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    fn initGitVersion(&self, version: serde_json::Value) {
        todo!()
    }

    pub(crate) fn setupConfig(&self, config: serde_json::Value) -> crate::composer::config::Config {
        todo!()
    }

    pub(crate) fn getDownloaderMock(&self, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, config: Option<crate::composer::config::Config>, executor: Option<crate::tests::composer::test::mock::process_executor_mock::ProcessExecutorMock>, filesystem: Option<crate::tests::composer::test::fixtures::functional::installed_versions2::vendor::symfony::filesystem::filesystem::Filesystem>) -> crate::composer::downloader::git_downloader::GitDownloader {
        todo!()
    }

    pub fn testDownloadForPackageWithoutSourceReference(&self) {
        todo!()
    }

    pub fn testDownload(&self) {
        todo!()
    }

    pub fn testDownloadWithCache(&self) {
        todo!()
    }

    pub fn testDownloadUsesVariousProtocolsAndSetsPushUrlForGithub(&self) {
        todo!()
    }

    pub fn pushUrlProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testDownloadAndSetPushUrlUseCustomVariousProtocolsForGithub(&self, protocols: Vec<serde_json::Value>, url: String, pushUrl: String) {
        todo!()
    }

    pub fn testDownloadThrowsRuntimeExceptionIfGitCommandFails(&self) {
        todo!()
    }

    pub fn testUpdateforPackageWithoutSourceReference(&self) {
        todo!()
    }

    pub fn testUpdate(&self) {
        todo!()
    }

    pub fn testUpdateWithNewRepoUrl(&self) {
        todo!()
    }

    pub fn testUpdateThrowsRuntimeExceptionIfGitCommandFails(&self) {
        todo!()
    }

    pub fn testUpdateDoesntThrowsRuntimeExceptionIfGitCommandFailsAtFirstButIsAbleToRecover(&self) {
        todo!()
    }

    pub fn testDowngradeShowsAppropriateMessage(&self) {
        todo!()
    }

    pub fn testNotUsingDowngradingWithReferences(&self) {
        todo!()
    }

    pub fn testRemove(&self) {
        todo!()
    }

    pub fn testGetInstallationSource(&self) {
        todo!()
    }

}

