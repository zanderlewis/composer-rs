// Namespace: Composer\Test\DependencyResolver

#[derive(Debug, Clone, Default)]
pub struct SecurityAdvisoryPoolFilterTest {
}

impl SecurityAdvisoryPoolFilterTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testFilterPackagesByAdvisories(&self) {
        todo!()
    }

    pub fn testDontFilterPackagesByIgnoredAdvisories(&self) {
        todo!()
    }

    pub fn testDontFilterPackagesWithBlockInsecureDisabled(&self) {
        todo!()
    }

    pub fn testDontFilterPackagesWithAbandonedPackage(&self) {
        todo!()
    }

    fn generateSecurityAdvisory(&self, packageName: String, cve: Option<String>, affectedVersions: String) -> Vec<serde_json::Value> {
        todo!()
    }

}

