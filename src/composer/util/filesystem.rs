// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct Filesystem {
}

impl Filesystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remove(&self, file: String) {
        todo!()
    }

    pub fn isDirEmpty(&self, dir: String) {
        todo!()
    }

    pub fn emptyDirectory(&self, dir: String, ensureDirectoryExists: bool) {
        todo!()
    }

    pub fn removeDirectory(&self, directory: String) {
        todo!()
    }

    pub fn removeDirectoryAsync(&self, directory: String) {
        todo!()
    }

    fn removeEdgeCases(&self, directory: String, fallbackToPhp: bool) -> Option<bool> {
        todo!()
    }

    pub fn removeDirectoryPhp(&self, directory: String) {
        todo!()
    }

    pub fn ensureDirectoryExists(&self, directory: String) {
        todo!()
    }

    pub fn unlink(&self, path: String) {
        todo!()
    }

    pub fn rmdir(&self, path: String) {
        todo!()
    }

    pub fn copyThenRemove(&self, source: String, target: String) {
        todo!()
    }

    pub fn copy(&self, source: String, target: String) {
        todo!()
    }

    pub fn rename(&self, source: String, target: String) {
        todo!()
    }

    pub fn findShortestPath(&self, from: String, to: String, directories: bool, preferRelative: bool) {
        todo!()
    }

    pub fn findShortestPathCode(&self, from: String, to: String, directories: bool, staticCode: bool, preferRelative: bool) {
        todo!()
    }

    pub fn isAbsolutePath(&self, path: String) {
        todo!()
    }

    pub fn size(&self, path: String) {
        todo!()
    }

    pub fn normalizePath(&self, path: String) {
        todo!()
    }

    pub fn trimTrailingSlash(path: String) {
        todo!()
    }

    pub fn isLocalPath(path: String) {
        todo!()
    }

    pub fn getPlatformPath(path: String) {
        todo!()
    }

    pub fn isReadable(path: String) {
        todo!()
    }

    pub(crate) fn directorySize(&self, directory: String) {
        todo!()
    }

    pub(crate) fn getProcess(&self) {
        todo!()
    }

    fn unlinkImplementation(&self, path: String) -> bool {
        todo!()
    }

    pub fn relativeSymlink(&self, target: String, link: String) {
        todo!()
    }

    pub fn isSymlinkedDirectory(&self, directory: String) {
        todo!()
    }

    fn unlinkSymlinkedDirectory(&self, directory: String) -> bool {
        todo!()
    }

    fn resolveSymlinkedDirectorySymlink(&self, pathname: String) -> String {
        todo!()
    }

    pub fn junction(&self, target: String, junction: String) {
        todo!()
    }

    pub fn isJunction(&self, junction: String) {
        todo!()
    }

    pub fn removeJunction(&self, junction: String) {
        todo!()
    }

    pub fn filePutContentsIfModified(&self, path: String, content: String) {
        todo!()
    }

    pub fn safeCopy(&self, source: String, target: String) {
        todo!()
    }

    fn filesAreEqual(&self, a: String, b: String) -> bool {
        todo!()
    }

}

