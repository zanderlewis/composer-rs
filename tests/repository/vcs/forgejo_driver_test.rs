// Namespace: Composer\Test\Repository\Vcs

#[derive(Debug, Clone, Default)]
pub struct ForgejoDriverTest {
}

impl ForgejoDriverTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testPublicRepository(&self) {
        todo!()
    }

    pub fn testGetBranches(&self) {
        todo!()
    }

    pub fn testGetTags(&self) {
        todo!()
    }

    pub fn testGetEmptyFileContent(&self) {
        todo!()
    }

    pub fn testSupports(&self, expected: bool, repoUrl: String) {
        todo!()
    }

    pub fn supportsProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    fn initializeDriver(&self, repoUrl: String) -> crate::composer::repository::vcs::forgejo_driver::ForgejoDriver {
        todo!()
    }

    fn expectInteractiveIO(&self, isInteractive: bool) {
        todo!()
    }

}

