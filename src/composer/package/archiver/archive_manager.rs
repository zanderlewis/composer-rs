// Namespace: Composer\Package\Archiver

#[derive(Debug, Clone, Default)]
pub struct ArchiveManager {
}

impl ArchiveManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn addArchiver(&self, archiver: Box<dyn crate::composer::package::archiver::archiver_interface::ArchiverInterface>) {
        todo!()
    }

    pub fn setOverwriteFiles(&self, overwriteFiles: bool) -> Self {
        todo!()
    }

    pub fn getPackageFilenameParts(&self, package: Box<dyn crate::composer::package::complete_package_interface::CompletePackageInterface>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPackageFilenameFromParts(&self, parts: Vec<serde_json::Value>) -> String {
        todo!()
    }

    pub fn getPackageFilename(&self, package: Box<dyn crate::composer::package::complete_package_interface::CompletePackageInterface>) -> String {
        todo!()
    }

    pub fn archive(&self, package: Box<dyn crate::composer::package::complete_package_interface::CompletePackageInterface>, format: String, targetDir: String, fileName: Option<String>, ignoreFilters: bool) -> String {
        todo!()
    }

    fn buildExcludePatterns(&self, parts: Vec<serde_json::Value>, formats: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getSupportedFormats(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

