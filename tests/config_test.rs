// Namespace: Composer\Test

#[derive(Debug, Clone, Default)]
pub struct ConfigTest {
}

impl ConfigTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testAddPackagistRepository(&self, expected: Vec<serde_json::Value>, localConfig: Vec<serde_json::Value>, systemConfig: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    pub fn dataAddPackagistRepository() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testPreferredInstallAsString(&self) {
        todo!()
    }

    pub fn testMergePreferredInstall(&self) {
        todo!()
    }

    pub fn testMergeGithubOauth(&self) {
        todo!()
    }

    pub fn testVarReplacement(&self) {
        todo!()
    }

    pub fn testRealpathReplacement(&self) {
        todo!()
    }

    pub fn testStreamWrapperDirs(&self) {
        todo!()
    }

    pub fn testFetchingRelativePaths(&self) {
        todo!()
    }

    pub fn testOverrideGithubProtocols(&self) {
        todo!()
    }

    pub fn testGitDisabledByDefaultInGithubProtocols(&self) {
        todo!()
    }

    pub fn testAllowedUrlsPass(&self, url: String) {
        todo!()
    }

    pub fn testProhibitedUrlsThrowException(&self, url: String) {
        todo!()
    }

    pub fn allowedUrlProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn prohibitedUrlProvider() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testProhibitedUrlsWarningVerifyPeer(&self) {
        todo!()
    }

    pub fn testDisableTlsCanBeOverridden(&self) {
        todo!()
    }

    pub fn testProcessTimeout(&self) {
        todo!()
    }

    pub fn testHtaccessProtect(&self) {
        todo!()
    }

    pub fn testGetSourceOfValue(&self) {
        todo!()
    }

    pub fn testGetSourceOfValueEnvVariables(&self) {
        todo!()
    }

    pub fn testAudit(&self) {
        todo!()
    }

    pub fn testGetDefaultsToAnEmptyArray(&self) {
        todo!()
    }

    pub fn testMergesPluginConfig(&self) {
        todo!()
    }

    pub fn testOverridesGlobalBooleanPluginsConfig(&self) {
        todo!()
    }

    pub fn testAllowsAllPluginsFromLocalBoolean(&self) {
        todo!()
    }

}

