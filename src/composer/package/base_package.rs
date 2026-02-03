// Namespace: Composer\Package

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct BasePackage {
}

impl BasePackage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getName(&self) -> String {
        todo!()
    }

    pub fn getPrettyName(&self) -> String {
        todo!()
    }

    pub fn getNames(&self, provides: serde_json::Value) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn setId(&self, id: i64) {
        todo!()
    }

    pub fn getId(&self) -> i64 {
        todo!()
    }

    pub fn setRepository(&self, repository: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>) {
        todo!()
    }

    pub fn getRepository(&self) -> Option<Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>> {
        todo!()
    }

    pub fn isPlatform(&self) -> bool {
        todo!()
    }

    pub fn getUniqueName(&self) -> String {
        todo!()
    }

    pub fn equals(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> bool {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

    pub fn getPrettyString(&self) -> String {
        todo!()
    }

    pub fn getFullPrettyVersion(&self, truncate: bool, displayMode: i64) -> String {
        todo!()
    }

    pub fn getStabilityPriority(&self) -> i64 {
        todo!()
    }

    pub fn __clone(&self) {
        todo!()
    }

    pub fn packageNameToRegexp(allowPattern: String, wrap: String) -> String {
        todo!()
    }

    pub fn packageNamesToRegexp(packageNames: Vec<serde_json::Value>, wrap: String) -> String {
        todo!()
    }

}

