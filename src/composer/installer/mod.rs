//! Auto-generated module declarations

pub mod binary_installer;
pub mod binary_presence_interface;
pub mod installation_manager;
pub mod installer_event;
pub mod installer_events;
pub mod installer_interface;
pub mod library_installer;
pub mod metapackage_installer;
pub mod noop_installer;
pub mod package_event;
pub mod package_events;
pub mod plugin_installer;
pub mod project_installer;
pub mod suggested_packages_reporter;

// --- Merged from parent module file ---

// Namespace: Composer

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Installer {
}

impl Installer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) -> i64 {
        todo!()
    }

    pub(crate) fn doUpdate(&self, localRepo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, doInstall: bool) -> i64 {
        todo!()
    }

    pub(crate) fn extractDevPackages(&self, lockTransaction: crate::composer::dependency_resolver::lock_transaction::LockTransaction, platformRepo: crate::composer::repository::platform_repository::PlatformRepository, aliases: Vec<serde_json::Value>, policy: Box<dyn crate::composer::dependency_resolver::policy_interface::PolicyInterface>, lockedRepository: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>) -> i64 {
        todo!()
    }

    pub(crate) fn doInstall(&self, localRepo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, alreadySolved: bool) -> i64 {
        todo!()
    }

    pub(crate) fn createPlatformRepo(&self, forUpdate: bool) -> crate::composer::repository::platform_repository::PlatformRepository {
        todo!()
    }

    fn createRepositorySet(&self, forUpdate: bool, platformRepo: crate::composer::repository::platform_repository::PlatformRepository, rootAliases: Vec<serde_json::Value>, lockedRepository: Option<Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>>) -> crate::composer::repository::repository_set::RepositorySet {
        todo!()
    }

    fn createPolicy(&self, forUpdate: bool, lockedRepo: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>) -> crate::composer::dependency_resolver::default_policy::DefaultPolicy {
        todo!()
    }

    fn createRequest(&self, rootPackage: Box<dyn crate::composer::package::root_package_interface::RootPackageInterface>, platformRepo: crate::composer::repository::platform_repository::PlatformRepository, lockedRepository: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>) -> crate::composer::dependency_resolver::request::Request {
        todo!()
    }

    fn requirePackagesForUpdate(&self, request: crate::composer::dependency_resolver::request::Request, lockedRepository: Option<crate::composer::repository::lock_array_repository::LockArrayRepository>, includeDevRequires: bool) {
        todo!()
    }

    fn getRootAliases(&self, forUpdate: bool) -> Vec<serde_json::Value> {
        todo!()
    }

    fn extractPlatformRequirements(&self, links: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn mockLocalRepositories(&self, rm: crate::composer::repository::repository_manager::RepositoryManager) {
        todo!()
    }

    fn createPoolOptimizer(&self, policy: Box<dyn crate::composer::dependency_resolver::policy_interface::PolicyInterface>) -> Option<crate::composer::dependency_resolver::pool_optimizer::PoolOptimizer> {
        todo!()
    }

    fn getAuditConfig(&self) -> crate::composer::advisory::audit_config::AuditConfig {
        todo!()
    }

    fn createSecurityAuditPoolFilter(&self) -> Option<crate::composer::dependency_resolver::security_advisory_pool_filter::SecurityAdvisoryPoolFilter> {
        todo!()
    }

    pub fn create(io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, composer: crate::composer::composer::Composer) -> Self {
        todo!()
    }

    pub fn setIgnoredTypes(&self, types: Vec<serde_json::Value>) -> Self {
        todo!()
    }

    pub fn setAllowedTypes(&self, types: Option<Vec<serde_json::Value>>) -> Self {
        todo!()
    }

    pub fn setAdditionalFixedRepository(&self, additionalFixedRepository: Box<dyn crate::composer::repository::repository_interface::RepositoryInterface>) -> Self {
        todo!()
    }

    pub fn setTemporaryConstraints(&self, constraints: Vec<serde_json::Value>) -> Self {
        todo!()
    }

    pub fn setDryRun(&self, dryRun: bool) -> Self {
        todo!()
    }

    pub fn isDryRun(&self) -> bool {
        todo!()
    }

    pub fn setDownloadOnly(&self, downloadOnly: bool) -> Self {
        todo!()
    }

    pub fn setPreferSource(&self, preferSource: bool) -> Self {
        todo!()
    }

    pub fn setPreferDist(&self, preferDist: bool) -> Self {
        todo!()
    }

    pub fn setOptimizeAutoloader(&self, optimizeAutoloader: bool) -> Self {
        todo!()
    }

    pub fn setClassMapAuthoritative(&self, classMapAuthoritative: bool) -> Self {
        todo!()
    }

    pub fn setApcuAutoloader(&self, apcuAutoloader: bool, apcuAutoloaderPrefix: Option<String>) -> Self {
        todo!()
    }

    pub fn setUpdate(&self, update: bool) -> Self {
        todo!()
    }

    pub fn setInstall(&self, install: bool) -> Self {
        todo!()
    }

    pub fn setDevMode(&self, devMode: bool) -> Self {
        todo!()
    }

    pub fn setDumpAutoloader(&self, dumpAutoloader: bool) -> Self {
        todo!()
    }

    pub fn setRunScripts(&self, runScripts: bool) -> Self {
        todo!()
    }

    pub fn setConfig(&self, config: crate::composer::config::Config) -> Self {
        todo!()
    }

    pub fn setVerbose(&self, verbose: bool) -> Self {
        todo!()
    }

    pub fn isVerbose(&self) -> bool {
        todo!()
    }

    pub fn setIgnorePlatformRequirements(&self, ignorePlatformReqs: serde_json::Value) -> Self {
        todo!()
    }

    pub fn setPlatformRequirementFilter(&self, platformRequirementFilter: Box<dyn crate::composer::filter::platform_requirement_filter::platform_requirement_filter_interface::PlatformRequirementFilterInterface>) -> Self {
        todo!()
    }

    pub fn setUpdateMirrors(&self, updateMirrors: bool) -> Self {
        todo!()
    }

    pub fn setUpdateAllowList(&self, packages: Vec<serde_json::Value>) -> Self {
        todo!()
    }

    pub fn setUpdateAllowTransitiveDependencies(&self, updateAllowTransitiveDependencies: i64) -> Self {
        todo!()
    }

    pub fn setPreferStable(&self, preferStable: bool) -> Self {
        todo!()
    }

    pub fn setPreferLowest(&self, preferLowest: bool) -> Self {
        todo!()
    }

    pub fn setMinimalUpdate(&self, minimalUpdate: bool) -> Self {
        todo!()
    }

    pub fn setWriteLock(&self, writeLock: bool) -> Self {
        todo!()
    }

    pub fn setExecuteOperations(&self, executeOperations: bool) -> Self {
        todo!()
    }

    pub fn setAudit(&self, audit: bool) -> Self {
        todo!()
    }

    pub fn setErrorOnAudit(&self, errorOnAudit: bool) -> Self {
        todo!()
    }

    pub fn setAuditFormat(&self, auditFormat: String) -> Self {
        todo!()
    }

    pub fn setAuditConfig(&self, auditConfig: crate::composer::advisory::audit_config::AuditConfig) -> Self {
        todo!()
    }

    pub fn disablePlugins(&self) -> Self {
        todo!()
    }

    pub fn setSuggestedPackagesReporter(&self, suggestedPackagesReporter: crate::composer::installer::suggested_packages_reporter::SuggestedPackagesReporter) -> Self {
        todo!()
    }

}

