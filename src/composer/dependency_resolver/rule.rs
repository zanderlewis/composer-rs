// Namespace: Composer\DependencyResolver

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Rule {
}

impl Rule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getLiterals(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getHash(&self) {
        todo!()
    }

    pub fn __toString(&self) -> String {
        todo!()
    }

    pub fn equals(&self, rule: crate::composer::dependency_resolver::rule::Rule) -> bool {
        todo!()
    }

    pub fn getReason(&self) -> i64 {
        todo!()
    }

    pub fn getReasonData(&self) {
        todo!()
    }

    pub fn getRequiredPackage(&self) -> Option<String> {
        todo!()
    }

    pub fn setType(&self, r#type: serde_json::Value) {
        todo!()
    }

    pub fn getType(&self) -> i64 {
        todo!()
    }

    pub fn disable(&self) {
        todo!()
    }

    pub fn enable(&self) {
        todo!()
    }

    pub fn isDisabled(&self) -> bool {
        todo!()
    }

    pub fn isEnabled(&self) -> bool {
        todo!()
    }

    pub fn isAssertion(&self) -> bool {
        todo!()
    }

    pub fn isCausedByLock(&self, repositorySet: crate::composer::repository::repository_set::RepositorySet, request: crate::composer::dependency_resolver::request::Request, pool: crate::composer::dependency_resolver::pool::Pool) -> bool {
        todo!()
    }

    pub fn getSourcePackage(&self, pool: crate::composer::dependency_resolver::pool::Pool) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    pub fn getPrettyString(&self, repositorySet: crate::composer::repository::repository_set::RepositorySet, request: crate::composer::dependency_resolver::request::Request, pool: crate::composer::dependency_resolver::pool::Pool, isVerbose: bool, installedMap: Vec<serde_json::Value>, learnedPool: Vec<serde_json::Value>) -> String {
        todo!()
    }

    pub(crate) fn formatPackagesUnique(&self, pool: crate::composer::dependency_resolver::pool::Pool, literalsOrPackages: Vec<serde_json::Value>, isVerbose: bool, constraint: Option<serde_json::Value>, useRemovedVersionGroup: bool) -> String {
        todo!()
    }

    fn deduplicateDefaultBranchAlias(&self, package: crate::composer::package::base_package::BasePackage) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

}

