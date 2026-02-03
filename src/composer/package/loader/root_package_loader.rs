// Namespace: Composer\Package\Loader

#[derive(Debug, Clone, Default)]
pub struct RootPackageLoader {
}

impl RootPackageLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self, config: Vec<serde_json::Value>, class: String, cwd: Option<String>) -> crate::composer::package::base_package::BasePackage {
        todo!()
    }

    fn extractAliases(&self, requires: Vec<serde_json::Value>, aliases: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn extractStabilityFlags(requires: Vec<serde_json::Value>, minimumStability: String, stabilityFlags: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn extractReferences(requires: Vec<serde_json::Value>, references: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

}

