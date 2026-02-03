// Namespace: Composer\Filter\PlatformRequirementFilter

pub trait PlatformRequirementFilterInterface {
    fn isIgnored(&self, req: String) -> bool;
    fn isUpperBoundIgnored(&self, req: String) -> bool;
}

