// Namespace: Composer\Test\Util

#[derive(Debug, Clone, Default)]
pub struct FilesystemTest {
}

impl FilesystemTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setUp(&self) {
        todo!()
    }

    pub(crate) fn tearDown(&self) {
        todo!()
    }

    pub fn testFindShortestPathCode(&self, a: String, b: String, directory: bool, expected: String, r#static: bool, preferRelative: bool) {
        todo!()
    }

    pub fn providePathCouplesAsCode() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testFindShortestPath(&self, a: String, b: String, expected: String, directory: bool, preferRelative: bool) {
        todo!()
    }

    pub fn providePathCouples() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testRemoveDirectoryPhp(&self) {
        todo!()
    }

    pub fn testFileSize(&self) {
        todo!()
    }

    pub fn testDirectorySize(&self) {
        todo!()
    }

    pub fn testNormalizePath(&self, expected: String, actual: String) {
        todo!()
    }

    pub fn provideNormalizedPaths() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testUnlinkSymlinkedDirectory(&self) {
        todo!()
    }

    pub fn testRemoveSymlinkedDirectoryWithTrailingSlash(&self) {
        todo!()
    }

    pub fn testJunctions(&self) {
        todo!()
    }

    pub fn testOverrideJunctions(&self) {
        todo!()
    }

    pub fn testCopy(&self) {
        todo!()
    }

    pub fn testCopyThenRemove(&self) {
        todo!()
    }

}

