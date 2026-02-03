// Namespace: Composer\Installer

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct SuggestedPackagesReporter {
}

impl SuggestedPackagesReporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getPackages(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn addPackage(&self, source: String, target: String, reason: String) -> crate::composer::installer::suggested_packages_reporter::SuggestedPackagesReporter {
        todo!()
    }

    pub fn addSuggestionsFromPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> crate::composer::installer::suggested_packages_reporter::SuggestedPackagesReporter {
        todo!()
    }

    pub fn output(&self, mode: i64, installedRepo: Option<crate::composer::repository::installed_repository::InstalledRepository>, onlyDependentsOf: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn outputMinimalistic(&self, installedRepo: Option<crate::composer::repository::installed_repository::InstalledRepository>, onlyDependentsOf: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    fn getFilteredSuggestions(&self, installedRepo: Option<crate::composer::repository::installed_repository::InstalledRepository>, onlyDependentsOf: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn escapeOutput(&self, string: String) -> String {
        todo!()
    }

    fn removeControlCharacters(&self, string: String) -> String {
        todo!()
    }

}

