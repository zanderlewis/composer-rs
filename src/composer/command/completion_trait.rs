// Namespace: Composer\Command

pub trait CompletionTrait {
    fn requireComposer(&self, disablePlugins: Option<bool>, disableScripts: Option<bool>) -> crate::composer::composer::Composer { todo!() }
    fn suggestPreferInstall(&self) -> Vec<serde_json::Value> { todo!() }
    fn suggestRootRequirement(&self) -> serde_json::Value { todo!() }
    fn suggestInstalledPackage(&self, includeRootPackage: bool, includePlatformPackages: bool) -> serde_json::Value { todo!() }
    fn suggestInstalledPackageTypes(&self, includeRootPackage: bool) -> serde_json::Value { todo!() }
    fn suggestAvailablePackage(&self, max: i64) -> serde_json::Value { todo!() }
    fn suggestAvailablePackageInclPlatform(&self) -> serde_json::Value { todo!() }
    fn suggestPlatformPackage(&self) -> serde_json::Value { todo!() }
}

