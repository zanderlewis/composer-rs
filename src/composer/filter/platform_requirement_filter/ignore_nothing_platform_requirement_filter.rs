// Namespace: Composer\Filter\PlatformRequirementFilter

#[derive(Debug, Clone, Default)]
pub struct IgnoreNothingPlatformRequirementFilter {
}

impl IgnoreNothingPlatformRequirementFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn isIgnored(&self, req: String) -> bool {
        todo!()
    }

    pub fn isUpperBoundIgnored(&self, req: String) -> bool {
        todo!()
    }

}

