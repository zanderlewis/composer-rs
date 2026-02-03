// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct UpdateCommand {
}

impl UpdateCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn getPackagesInteractively(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, input: serde_json::Value, output: serde_json::Value, composer: crate::composer::composer::Composer, packages: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    fn createVersionSelector(&self, composer: crate::composer::composer::Composer) -> crate::composer::package::version::version_selector::VersionSelector {
        todo!()
    }

}

