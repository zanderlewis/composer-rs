// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct AuthHelperTest {
}

impl AuthHelperTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn setUp(&self) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithoutAuthCredentials(&self) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithBearerPassword(&self) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithGithubToken(&self) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithGitlabOathToken(&self) {
        todo!()
    }

    pub fn testAddAuthenticationOptionsForClientCertificate(&self) {
        todo!()
    }

    pub fn gitlabPrivateTokenProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithGitlabPrivateToken(&self, password: String) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithBitbucketOathToken(&self) {
        todo!()
    }

    pub fn bitbucketPublicUrlProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithBitbucketPublicUrl(&self, url: String) {
        todo!()
    }

    pub fn basicHttpAuthenticationProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithBasicHttpAuthentication(&self, url: String, origin: String, auth: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderWithCustomHeaders(&self) {
        todo!()
    }

    pub fn testIsPublicBitBucketDownloadWithBitbucketPublicUrl(&self, url: String) {
        todo!()
    }

    pub fn testIsPublicBitBucketDownloadWithNonBitbucketPublicUrl(&self) {
        todo!()
    }

    pub fn testStoreAuthAutomatically(&self) {
        todo!()
    }

    pub fn testStoreAuthWithPromptYesAnswer(&self) {
        todo!()
    }

    pub fn testStoreAuthWithPromptNoAnswer(&self) {
        todo!()
    }

    pub fn testStoreAuthWithPromptInvalidAnswer(&self) {
        todo!()
    }

    pub fn testPromptAuthIfNeededGitLabNoAuthChange(&self) {
        todo!()
    }

    pub fn testPromptAuthIfNeededMultipleBitbucketDownloads(&self) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderIsWorking(&self, url: String, origin: String, auth: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testAddAuthenticationHeaderDeprecation(&self) {
        todo!()
    }

    fn expectsAuthentication(&self, origin: String, auth: Vec<serde_json::Value>) {
        todo!()
    }

}

