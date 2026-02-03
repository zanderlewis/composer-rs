// Namespace: Composer\Util

#[derive(Debug, Clone, Default)]
pub struct SyncHelper {
}

impl SyncHelper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn downloadAndInstallPackageSync(r#loop: crate::composer::util::loop_::Loop, downloader: serde_json::Value, path: String, package: Box<dyn crate::composer::package::package_interface::PackageInterface>, prevPackage: Option<Box<dyn crate::composer::package::package_interface::PackageInterface>>) {
        todo!()
    }

    pub fn r#await(r#loop: crate::composer::util::loop_::Loop, promise: Option<serde_json::Value>) {
        todo!()
    }

}

