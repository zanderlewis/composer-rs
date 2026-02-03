// Namespace: Composer\Test\Downloader

#[derive(Debug, Clone, Default)]
pub struct DownloadManagerTest {
}

impl DownloadManagerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub fn testSetGetDownloader(&self) {
        todo!()
    }

    pub fn testGetDownloaderForIncorrectlyInstalledPackage(&self) {
        todo!()
    }

    pub fn testGetDownloaderForCorrectlyInstalledDistPackage(&self) {
        todo!()
    }

    pub fn testGetDownloaderForIncorrectlyInstalledDistPackage(&self) {
        todo!()
    }

    pub fn testGetDownloaderForCorrectlyInstalledSourcePackage(&self) {
        todo!()
    }

    pub fn testGetDownloaderForIncorrectlyInstalledSourcePackage(&self) {
        todo!()
    }

    pub fn testGetDownloaderForMetapackage(&self) {
        todo!()
    }

    pub fn testFullPackageDownload(&self) {
        todo!()
    }

    pub fn testFullPackageDownloadFailover(&self) {
        todo!()
    }

    pub fn testBadPackageDownload(&self) {
        todo!()
    }

    pub fn testDistOnlyPackageDownload(&self) {
        todo!()
    }

    pub fn testSourceOnlyPackageDownload(&self) {
        todo!()
    }

    pub fn testMetapackagePackageDownload(&self) {
        todo!()
    }

    pub fn testFullPackageDownloadWithSourcePreferred(&self) {
        todo!()
    }

    pub fn testDistOnlyPackageDownloadWithSourcePreferred(&self) {
        todo!()
    }

    pub fn testSourceOnlyPackageDownloadWithSourcePreferred(&self) {
        todo!()
    }

    pub fn testBadPackageDownloadWithSourcePreferred(&self) {
        todo!()
    }

    pub fn testUpdateDistWithEqualTypes(&self) {
        todo!()
    }

    pub fn testUpdateDistWithNotEqualTypes(&self) {
        todo!()
    }

    pub fn testGetAvailableSourcesUpdateSticksToSameSource(&self, prevPkgSource: Option<String>, prevPkgIsDev: Option<bool>, targetAvailable: Vec<serde_json::Value>, targetIsDev: bool, expected: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn updatesProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testUpdateMetapackage(&self) {
        todo!()
    }

    pub fn testRemove(&self) {
        todo!()
    }

    pub fn testMetapackageRemove(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithoutPreferenceDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithoutPreferenceNoDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithoutMatchDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithoutMatchNoDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithMatchAutoDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithMatchAutoNoDev(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithMatchSource(&self) {
        todo!()
    }

    pub fn testInstallPreferenceWithMatchDist(&self) {
        todo!()
    }

    fn createDownloaderMock(&self) {
        todo!()
    }

    fn createPackageMock(&self) {
        todo!()
    }

}

