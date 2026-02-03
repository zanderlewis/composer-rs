// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct GitTest {
}

impl GitTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub fn testRunCommandPublicGitHubRepositoryNotInitialClone(&self, protocol: String, expectedUrl: String) {
        todo!()
    }

    pub fn publicGithubNoCredentialsProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRunCommandPrivateGitHubRepositoryNotInitialCloneNotInteractiveWithoutAuthentication(&self) {
        todo!()
    }

    pub fn testRunCommandPrivateGitHubRepositoryNotInitialCloneNotInteractiveWithAuthentication(&self, gitUrl: String, protocol: String, gitHubToken: String, expectedUrl: String, expectedFailuresBeforeSuccess: i64) {
        todo!()
    }

    pub fn testRunCommandPrivateBitbucketRepositoryNotInitialCloneNotInteractiveWithAuthentication(&self, gitUrl: String, bitbucketToken: Option<String>, expectedUrl: String, expectedFailuresBeforeSuccess: i64, bitbucket_git_auth_calls: i64) {
        todo!()
    }

    pub fn testRunCommandPrivateBitbucketRepositoryNotInitialCloneInteractiveWithOauth(&self, gitUrl: String, expectedUrl: String, initial_config: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn privateBitbucketWithOauthProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn privateBitbucketWithCredentialsProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn privateGithubWithCredentialsProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    fn mockConfig(&self, protocol: String) {
        todo!()
    }

}

