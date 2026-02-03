// Namespace: Composer\Package

#[derive(Debug, Clone, Default)]
pub struct Locker {
}

impl Locker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getJsonFile(&self) -> crate::composer::json::json_file::JsonFile {
        todo!()
    }

    pub fn getContentHash(composerFileContents: String) -> String {
        todo!()
    }

    pub fn isLocked(&self) -> bool {
        todo!()
    }

    pub fn isFresh(&self) -> bool {
        todo!()
    }

    pub fn getLockedRepository(&self, withDevReqs: bool) -> crate::composer::repository::lock_array_repository::LockArrayRepository {
        todo!()
    }

    pub fn getDevPackageNames(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPlatformRequirements(&self, withDevReqs: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getMinimumStability(&self) -> String {
        todo!()
    }

    pub fn getStabilityFlags(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPreferStable(&self) -> Option<bool> {
        todo!()
    }

    pub fn getPreferLowest(&self) -> Option<bool> {
        todo!()
    }

    pub fn getPlatformOverrides(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getAliases(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPluginApi(&self) {
        todo!()
    }

    pub fn getLockData(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setLockData(&self, packages: Vec<serde_json::Value>, devPackages: Option<Vec<serde_json::Value>>, platformReqs: Vec<serde_json::Value>, platformDevReqs: Vec<serde_json::Value>, aliases: Vec<serde_json::Value>, minimumStability: String, stabilityFlags: Vec<serde_json::Value>, preferStable: bool, preferLowest: bool, platformOverrides: Vec<serde_json::Value>, write: bool) -> bool {
        todo!()
    }

    pub fn updateHash(&self, composerJson: crate::composer::json::json_file::JsonFile, dataProcessor: Option<Box<dyn Fn()>>) {
        todo!()
    }

    fn fixupJsonDataType(&self, lockData: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn lockPackages(&self, packages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getPackageTime(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<String> {
        todo!()
    }

    pub fn getMissingRequirementInfo(&self, package: Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>, includeDev: bool) -> Vec<serde_json::Value> {
        todo!()
    }

}

