// Namespace: Composer\Downloader

pub trait DvcsDownloaderInterface {
    fn getUnpushedChanges(&self, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, path: String) -> Option<String>;
}

