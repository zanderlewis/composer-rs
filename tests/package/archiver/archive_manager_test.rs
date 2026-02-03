// Namespace: Composer\Test\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct ArchiveManagerTest {
}

impl ArchiveManagerTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub fn testUnknownFormat(&self) {
        todo!()
    }

    pub fn testArchiveTar(&self) {
        todo!()
    }

    pub fn testArchiveCustomFileName(&self) {
        todo!()
    }

    pub fn testGetPackageFilenameParts(&self) {
        todo!()
    }

    pub fn testGetPackageFilename(&self) {
        todo!()
    }

    pub(crate) fn getTargetName(&self, package: crate::composer::package::complete_package::CompletePackage, format: String, fileName: Option<String>) -> String {
        todo!()
    }

    pub(crate) fn setupGitRepo(&self) {
        todo!()
    }

}

