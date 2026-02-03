// Namespace: Composer\Downloader

pub trait ChangeReportInterface {
    fn getLocalChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String>;
}

