// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Request {
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn requireName(&self, packageName: String, constraint: Option<serde_json::Value>) {
        todo!()
    }

    pub fn fixPackage(&self, package: crate::composer::package::base_package::BasePackage) {
        todo!()
    }

    pub fn lockPackage(&self, package: crate::composer::package::base_package::BasePackage) {
        todo!()
    }

    pub fn fixLockedPackage(&self, package: crate::composer::package::base_package::BasePackage) {
        todo!()
    }

    pub fn unlockPackage(&self, package: crate::composer::package::base_package::BasePackage) {
        todo!()
    }

    pub fn setUpdateAllowList(&self, updateAllowList: Vec<serde_json::Value>, updateAllowTransitiveDependencies: serde_json::Value) {
        todo!()
    }

    pub fn getUpdateAllowList(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getUpdateAllowTransitiveDependencies(&self) -> bool {
        todo!()
    }

    pub fn getUpdateAllowTransitiveRootDependencies(&self) -> bool {
        todo!()
    }

    pub fn getRequires(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getFixedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isFixedPackage(&self, package: crate::composer::package::base_package::BasePackage) -> bool {
        todo!()
    }

    pub fn getLockedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isLockedPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> bool {
        todo!()
    }

    pub fn getFixedOrLockedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getPresentMap(&self, packageIds: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getFixedPackagesMap(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getLockedRepository(&self) -> Option<crate::composer::repository::lock_array_repository::LockArrayRepository> {
        todo!()
    }

    pub fn restrictPackages(&self, names: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getRestrictedPackages(&self) -> Option<Vec<serde_json::Value>> {
        todo!()
    }

}

