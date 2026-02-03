// Namespace: Composer\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct Pool {
}

impl Pool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getRemovedVersions(&self, name: String, constraint: serde_json::Value) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getAllRemovedVersions(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getRemovedVersionsByPackage(&self, objectHash: String) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getAllRemovedVersionsByPackage(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isSecurityRemovedPackageVersion(&self, packageName: String, constraint: Option<serde_json::Value>) -> bool {
        todo!()
    }

    pub fn getSecurityAdvisoryIdentifiersForPackageVersion(&self, packageName: String, constraint: Option<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn isAbandonedRemovedPackageVersion(&self, packageName: String, constraint: Option<serde_json::Value>) -> bool {
        todo!()
    }

    pub fn getAllSecurityRemovedPackageVersions(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getAllAbandonedRemovedPackageVersions(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    fn setPackages(&self, packages: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn getPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn packageById(&self, id: i64) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn count(&self) -> i64 {
        todo!()
    }

    pub fn whatProvides(&self, name: String, constraint: Option<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn computeWhatProvides(&self, name: String, constraint: Option<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn literalToPackage(&self, literal: i64) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn literalToPrettyString(&self, literal: i64, installedMap: Vec<serde_json::Value>) -> String {
        todo!()
    }

    pub fn r#match(&self, candidate: crate::composer::package::base_package::BasePackage, name: String, constraint: Option<serde_json::Value>) -> bool {
        todo!()
    }

    pub fn isUnacceptableFixedOrLockedPackage(&self, package: crate::composer::package::base_package::BasePackage) -> bool {
        todo!()
    }

    pub fn getUnacceptableFixedOrLockedPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

}

