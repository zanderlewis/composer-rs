// Namespace: Composer\Test\Repository

#[derive(Debug, Clone, Default)]
pub struct RepositoryUtilsTest {
}

impl RepositoryUtilsTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testFilterRequiredPackages(&self, pkgs: Vec<serde_json::Value>, requirer: Box<dyn crate::composer::package::package_interface::PackageInterface>, expected: Vec<serde_json::Value>, includeRequireDev: bool) {
        todo!()
    }

    fn getPackages() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn provideFilterRequireTests() -> serde_json::Value {
        todo!()
    }

}

