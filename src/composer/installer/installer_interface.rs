// Namespace: Composer\Installer

pub trait InstallerInterface {
    fn supports(&self, packageType: String);
    fn isInstalled(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn download(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>);
    fn prepare(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>);
    fn install(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn update(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, initial: Box<dyn crate::composer::package::package_interface::PackageInterface>, target: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn uninstall(&self, repo: Box<dyn crate::composer::repository::installed_repository_interface::InstalledRepositoryInterface>, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
    fn cleanup(&self, r#type: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>);
    fn getInstallPath(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>);
}

