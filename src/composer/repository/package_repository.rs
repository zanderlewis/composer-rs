// Namespace: Composer\Repository

#[derive(Debug, Clone, Default)]
pub struct PackageRepository {
}

impl PackageRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn initialize(&self) {
        todo!()
    }

    pub fn getRepoName(&self) -> String {
        todo!()
    }

    pub fn hasSecurityAdvisories(&self) -> bool {
        todo!()
    }

    pub fn getSecurityAdvisories(&self, packageConstraintMap: Vec<serde_json::Value>, allowPartialAdvisories: bool) -> Vec<serde_json::Value> {
        todo!()
    }

}

