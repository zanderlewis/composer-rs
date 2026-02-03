// Namespace: Composer\Advisory

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
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Auditor {
}

impl Auditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn audit(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, repoSet: crate::composer::repository::repository_set::RepositorySet, packages: Vec<serde_json::Value>, format: String, warningOnly: bool, ignoreList: Vec<serde_json::Value>, abandoned: String, ignoredSeverities: Vec<serde_json::Value>, ignoreUnreachable: bool, ignoreAbandoned: Vec<serde_json::Value>) -> i64 {
        todo!()
    }

    pub fn needsCompleteAdvisoryLoad(&self, advisories: Vec<serde_json::Value>, ignoreList: Vec<serde_json::Value>) -> bool {
        todo!()
    }

    pub fn filterAbandonedPackages(&self, packages: Vec<serde_json::Value>, ignoreAbandoned: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn processAdvisories(&self, allAdvisories: Vec<serde_json::Value>, ignoreList: Vec<serde_json::Value>, ignoredSeverities: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn countAdvisories(&self, advisories: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn outputAdvisories(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, advisories: Vec<serde_json::Value>, format: String) {
        todo!()
    }

    fn outputAdvisoriesTable(&self, io: crate::composer::i_o::console_i_o::ConsoleIO, advisories: Vec<serde_json::Value>) {
        todo!()
    }

    fn outputAdvisoriesPlain(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, advisories: Vec<serde_json::Value>) {
        todo!()
    }

    fn outputAbandonedPackages(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, packages: Vec<serde_json::Value>, format: String) {
        todo!()
    }

    fn getPackageNameWithLink(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> String {
        todo!()
    }

    fn getSeverity(&self, advisory: crate::composer::advisory::security_advisory::SecurityAdvisory) -> String {
        todo!()
    }

    fn getAdvisoryId(&self, advisory: crate::composer::advisory::security_advisory::SecurityAdvisory) -> String {
        todo!()
    }

    fn getCVE(&self, advisory: crate::composer::advisory::security_advisory::SecurityAdvisory) -> String {
        todo!()
    }

    fn getURL(&self, advisory: crate::composer::advisory::security_advisory::SecurityAdvisory) -> String {
        todo!()
    }

    fn calculateBitmask(&self, hasVulnerablePackages: bool, hasAbandonedPackages: bool) -> i64 {
        todo!()
    }

}

