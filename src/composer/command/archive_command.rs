// Namespace: Composer\Command

// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct ArchiveCommand {
}

impl ArchiveCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    pub(crate) fn archive(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, config: crate::composer::config::Config, packageName: Option<String>, version: Option<String>, format: String, dest: String, fileName: Option<String>, ignoreFilters: bool, composer: Option<crate::composer::composer::Composer>) -> i64 {
        todo!()
    }

    pub(crate) fn selectPackage(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, packageName: String, version: Option<String>) {
        todo!()
    }

}

