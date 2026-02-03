// Namespace: Composer\Test\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct RootPackageLoaderTest {
}

impl RootPackageLoaderTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn loadPackage(&self, data: Vec<serde_json::Value>) -> Box<dyn crate::composer::package::package_interface::PackageInterface> {
        todo!()
    }

    pub fn testStabilityFlagsParsing(&self) {
        todo!()
    }

    pub fn testNoVersionIsVisibleInPrettyVersion(&self) {
        todo!()
    }

    pub fn testPrettyVersionForRootPackageInVersionBranch(&self) {
        todo!()
    }

    pub fn testFeatureBranchPrettyVersion(&self) {
        todo!()
    }

    pub fn testNonFeatureBranchPrettyVersion(&self) {
        todo!()
    }

}

