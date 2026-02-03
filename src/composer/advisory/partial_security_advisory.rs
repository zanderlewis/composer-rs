// Namespace: Composer\Advisory

#[derive(Debug, Clone, Default)]
pub struct PartialSecurityAdvisory {
}

impl PartialSecurityAdvisory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(packageName: String, data: Vec<serde_json::Value>, parser: crate::composer::package::version::version_parser::VersionParser) -> Self {
        todo!()
    }

    pub fn jsonSerialize(&self) {
        todo!()
    }

}

