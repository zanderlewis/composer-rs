// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct BaseConfigCommand {
}

impl BaseConfigCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn initialize(&self, input: serde_json::Value, output: serde_json::Value) {
        todo!()
    }

    pub(crate) fn getComposerConfigFile(&self, input: serde_json::Value, config: crate::composer::config::Config) -> String {
        todo!()
    }

    pub(crate) fn getAuthConfigFile(&self, input: serde_json::Value, config: crate::composer::config::Config) -> String {
        todo!()
    }

}

