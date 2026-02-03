// Namespace: Composer\Test\Repository

#[derive(Debug, Clone, Default)]
pub struct PlatformRepositoryTest {
}

impl PlatformRepositoryTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testHhvmPackage(&self) {
        todo!()
    }

    pub fn providePhpFlavorTestCases() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testPhpVersion(&self, constants: Vec<serde_json::Value>, packages: Vec<serde_json::Value>, functions: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testInetPtonRegression(&self) {
        todo!()
    }

    pub fn provideLibraryTestCases() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testLibraryInformation(&self, extensions: serde_json::Value, info: Option<String>, expectations: Vec<serde_json::Value>, functions: Vec<serde_json::Value>, constants: Vec<serde_json::Value>, classDefinitions: Vec<serde_json::Value>) {
        todo!()
    }

    fn assertPackageLinks(&self, context: String, expectedLinks: Vec<serde_json::Value>, sourcePackage: Box<dyn crate::composer::package::package_interface::PackageInterface>, links: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testComposerPlatformVersion(&self) {
        todo!()
    }

    pub fn providePlatformPackages() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testValidPlatformPackages(&self, packageName: String, expectation: bool) {
        todo!()
    }

}

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ResourceBundleStub {
}

impl ResourceBundleStub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(locale: String, bundleName: String, fallback: bool) -> crate::tests::composer::test::repository::platform_repository_test::ResourceBundleStub {
        todo!()
    }

    pub fn get(&self, field: serde_json::Value) -> String {
        todo!()
    }

}

#[derive(Debug, Clone, Default)]
pub struct ImagickStub {
}

impl ImagickStub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getVersion(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

