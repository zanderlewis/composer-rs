// Namespace: Composer\Downloader

pub trait VcsCapableDownloaderInterface {
    fn getVcsReference(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String>;
}

