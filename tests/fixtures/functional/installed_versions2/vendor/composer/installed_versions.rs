// Namespace: Composer

#[derive(Debug, Clone, Default)]
pub struct InstalledVersions {
}

impl InstalledVersions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInstalledPackages() {
        todo!()
    }

    pub fn isInstalled(packageName: serde_json::Value) {
        todo!()
    }

    pub fn satisfies(parser: crate::composer::package::version::version_parser::VersionParser, packageName: serde_json::Value, constraint: serde_json::Value) {
        todo!()
    }

    pub fn getVersionRanges(packageName: serde_json::Value) {
        todo!()
    }

    pub fn getVersion(packageName: serde_json::Value) {
        todo!()
    }

    pub fn getPrettyVersion(packageName: serde_json::Value) {
        todo!()
    }

    pub fn getReference(packageName: serde_json::Value) {
        todo!()
    }

    pub fn getRootPackage() {
        todo!()
    }

    pub fn getRawData() {
        todo!()
    }

    pub fn reload(data: serde_json::Value) {
        todo!()
    }

    fn getInstalled() {
        todo!()
    }

}

