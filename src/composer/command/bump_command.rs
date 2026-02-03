// Namespace: Composer\Command

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct BumpCommand {
}

impl BumpCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    pub fn doBump(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, devOnly: bool, noDevOnly: bool, dryRun: bool, packagesFilter: Vec<serde_json::Value>, devOnlyFlagHint: String) -> i64 {
        todo!()
    }

    fn updateFileCleanly(&self, json: crate::composer::json::json_file::JsonFile, updates: Vec<serde_json::Value>) -> bool {
        todo!()
    }

}

