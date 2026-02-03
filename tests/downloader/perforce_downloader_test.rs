// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct PerforceDownloaderTest {
}

impl PerforceDownloaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub(crate) fn getConfig(&self, configOptions: Vec<serde_json::Value>, useEnvironment: bool) -> crate::composer::config::Config {
        todo!()
    }

    pub(crate) fn getMockIoInterface(&self) {
        todo!()
    }

    pub(crate) fn getMockPackageInterface(&self, repository: crate::composer::repository::vcs_repository::VcsRepository) {
        todo!()
    }

    pub(crate) fn getRepoConfig(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub(crate) fn getMockRepository(&self, repoConfig: Vec<serde_json::Value>, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, config: crate::composer::config::Config) {
        todo!()
    }

    pub fn testInitPerforceInstantiatesANewPerforceObject(&self) {
        todo!()
    }

    pub fn testInitPerforceDoesNothingIfPerforceAlreadySet(&self) {
        todo!()
    }

    pub fn testDoInstallWithTag(&self) {
        todo!()
    }

    pub fn testDoInstallWithNoTag(&self) {
        todo!()
    }

}

