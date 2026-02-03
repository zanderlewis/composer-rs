// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct PackageInfo {
}

impl PackageInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getViewSourceUrl(package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<String> {
        todo!()
    }

    pub fn getViewSourceOrHomepageUrl(package: Box<dyn crate::composer::package::package_interface::PackageInterface>) -> Option<String> {
        todo!()
    }

}

