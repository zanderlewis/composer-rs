// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct RemoteFilesystemTest {
}

impl RemoteFilesystemTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testGetOptionsForUrl(&self) {
        todo!()
    }

    pub fn testGetOptionsForUrlWithAuthorization(&self) {
        todo!()
    }

    pub fn testGetOptionsForUrlWithStreamOptions(&self) {
        todo!()
    }

    pub fn testGetOptionsForUrlWithCallOptionsKeepsHeader(&self) {
        todo!()
    }

    pub fn testCallbackGetFileSize(&self) {
        todo!()
    }

    pub fn testCallbackGetNotifyProgress(&self) {
        todo!()
    }

    pub fn testCallbackGetPassesThrough404(&self) {
        todo!()
    }

    pub fn testGetContents(&self) {
        todo!()
    }

    pub fn testCopy(&self) {
        todo!()
    }

    pub fn testCopyWithNoRetryOnFailure(&self) {
        todo!()
    }

    pub fn testCopyWithSuccessOnRetry(&self) {
        todo!()
    }

    pub fn testGetOptionsForUrlCreatesSecureTlsDefaults(&self) {
        todo!()
    }

    pub fn provideBitbucketPublicDownloadUrls() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testBitBucketPublicDownload(&self, url: String, contents: String) {
        todo!()
    }

    fn callGetOptionsForUrl(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, args: Vec<serde_json::Value>, options: Vec<serde_json::Value>, fileUrl: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getConfigMock(&self) {
        todo!()
    }

    fn callCallbackGet(&self, fs: crate::composer::util::remote_filesystem::RemoteFilesystem, notificationCode: i64, severity: i64, message: String, messageCode: i64, bytesTransferred: i64, bytesMax: i64) {
        todo!()
    }

    fn setAttribute(&self, object: serde_json::Value, attribute: String, value: serde_json::Value) {
        todo!()
    }

    fn assertAttributeEqualsCustom(&self, value: serde_json::Value, attribute: String, object: serde_json::Value) {
        todo!()
    }

    fn getIOInterfaceMock(&self) {
        todo!()
    }

    fn getRemoteFilesystemWithMockedMethods(&self, mockedMethods: Vec<serde_json::Value>, authHelper: Option<crate::composer::util::auth_helper::AuthHelper>) {
        todo!()
    }

    fn getAuthHelperWithMockedMethods(&self, mockedMethods: Vec<serde_json::Value>) {
        todo!()
    }

}

