// Namespace: Composer\Filter\PlatformRequirementFilter

#[derive(Debug, Clone, Default)]
pub struct IgnoreListPlatformRequirementFilter {
}

impl IgnoreListPlatformRequirementFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn isIgnored(&self, req: String) -> bool {
        todo!()
    }

    pub fn isUpperBoundIgnored(&self, req: String) -> bool {
        todo!()
    }

    pub fn filterConstraint(&self, req: String, constraint: serde_json::Value, allowUpperBoundOverride: bool) -> serde_json::Value {
        todo!()
    }

}

