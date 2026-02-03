// Namespace: Composer\Test\Repository\Vcs

#[derive(Debug, Clone, Default)]
pub struct HgDriverTest {
}

impl HgDriverTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testSupports(&self, repositoryUrl: String) {
        todo!()
    }

    pub fn supportsDataProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGetBranchesFilterInvalidBranchNames(&self) {
        todo!()
    }

    pub fn testFileGetContentInvalidIdentifier(&self) {
        todo!()
    }

}

