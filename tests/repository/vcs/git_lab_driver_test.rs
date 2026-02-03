// Namespace: Composer\Test\Repository\Vcs

#[derive(Debug, Clone, Default)]
pub struct GitLabDriverTest {
}

impl GitLabDriverTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn provideInitializeUrls() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testInitialize(&self, url: String, apiUrl: String) -> crate::composer::repository::vcs::git_lab_driver::GitLabDriver {
        todo!()
    }

    pub fn testInitializePublicProject(&self, url: String, apiUrl: String) -> crate::composer::repository::vcs::git_lab_driver::GitLabDriver {
        todo!()
    }

    pub fn testInitializePublicProjectAsAnonymous(&self, url: String, apiUrl: String) -> crate::composer::repository::vcs::git_lab_driver::GitLabDriver {
        todo!()
    }

    pub fn testInitializeWithPortNumber(&self) {
        todo!()
    }

    pub fn testInvalidSupportData(&self) {
        todo!()
    }

    pub fn testGetDist(&self) {
        todo!()
    }

    pub fn testGetSource(&self) {
        todo!()
    }

    pub fn testGetSource_GivenPublicProject(&self) {
        todo!()
    }

    pub fn testGetTags(&self) {
        todo!()
    }

    pub fn testGetPaginatedRefs(&self) {
        todo!()
    }

    pub fn testGetBranches(&self) {
        todo!()
    }

    pub fn testSupports(&self, url: String, expected: bool) {
        todo!()
    }

    pub fn dataForTestSupports() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testGitlabSubDirectory(&self) {
        todo!()
    }

    pub fn testGitlabSubGroup(&self) {
        todo!()
    }

    pub fn testGitlabSubDirectorySubGroup(&self) {
        todo!()
    }

    pub fn testForwardsOptions(&self) {
        todo!()
    }

    pub fn testProtocolOverrideRepositoryUrlGeneration(&self) {
        todo!()
    }

    pub(crate) fn setAttribute(&self, object: serde_json::Value, attribute: String, value: serde_json::Value) {
        todo!()
    }

}

