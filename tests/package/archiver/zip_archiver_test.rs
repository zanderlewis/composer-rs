// Namespace: Composer\Test\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct ZipArchiverTest {
}

impl ZipArchiverTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testSimpleFiles(&self) {
        todo!()
    }

    pub fn testGitignoreExcludeNegation(&self, include: String) {
        todo!()
    }

    pub fn provideGitignoreExcludeNegationTestCases() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testFolderWithBackslashes(&self) {
        todo!()
    }

    pub(crate) fn assertZipArchive(&self, files: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn setupDummyRepo(&self, files: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn writeFile(&self, path: String, content: String, currentWorkDir: String) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

}

