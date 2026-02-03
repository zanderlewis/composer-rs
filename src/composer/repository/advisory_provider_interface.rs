// Namespace: Composer\Repository

pub trait AdvisoryProviderInterface {
    fn hasSecurityAdvisories(&self) -> bool;
    fn getSecurityAdvisories(&self, packageConstraintMap: Vec<serde_json::Value>, allowPartialAdvisories: bool) -> Vec<serde_json::Value>;
}

