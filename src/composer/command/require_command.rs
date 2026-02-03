// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct RequireCommand {
}

impl RequireCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn getInconsistentRequireKeys(&self, newRequirements: Vec<serde_json::Value>, requireKey: String) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getPackagesByRequireKey(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    fn doUpdate(&self, input: serde_json::Value, output: serde_json::Value, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, requirements: Vec<serde_json::Value>, requireKey: String, removeKey: String) -> i64 {
        todo!()
    }

    fn updateRequirementsAfterResolution(&self, requirementsToUpdate: Vec<serde_json::Value>, requireKey: String, removeKey: String, sortPackages: bool, dryRun: bool, fixed: bool) -> i64 {
        todo!()
    }

    fn updateFile(&self, json: crate::composer::json::json_file::JsonFile, new: Vec<serde_json::Value>, requireKey: String, removeKey: String, sortPackages: bool) {
        todo!()
    }

    fn updateFileCleanly(&self, json: crate::composer::json::json_file::JsonFile, new: Vec<serde_json::Value>, requireKey: String, removeKey: String, sortPackages: bool) -> bool {
        todo!()
    }

    pub(crate) fn interact(&self, input: serde_json::Value, output: serde_json::Value) {
        todo!()
    }

    fn revertComposerFile(&self) {
        todo!()
    }

}

