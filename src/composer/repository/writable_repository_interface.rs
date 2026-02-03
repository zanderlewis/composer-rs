// Namespace: Composer\Repository

pub trait WritableRepositoryInterface {
    fn write(&self, devMode: bool, installationManager: crate::composer::installer::installation_manager::InstallationManager);
    fn addPackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn removePackage(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn getCanonicalPackages(&self);
    fn reload(&self);
    fn setDevPackageNames(&self, devPackageNames: Vec<serde_json::Value>);
    fn getDevPackageNames(&self);
}

