// Namespace: Composer\Console

#[derive(Debug, Clone, Default)]
pub struct Application {
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn __destruct(&self) {
        todo!()
    }

    pub fn run(&self, input: Option<serde_json::Value>, output: Option<serde_json::Value>) -> i64 {
        todo!()
    }

    pub fn doRun(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn getNewWorkingDir(&self, input: serde_json::Value) -> Option<String> {
        todo!()
    }

    fn hintCommonErrors(&self, exception: serde_json::Value, output: serde_json::Value) {
        todo!()
    }

    pub fn getComposer(&self, required: bool, disablePlugins: Option<bool>, disableScripts: Option<bool>) -> Option<crate::composer::composer::Composer> {
        todo!()
    }

    pub fn resetComposer(&self) {
        todo!()
    }

    pub fn getIO(&self) -> Box<dyn crate::composer::i_o::i_o_interface::IOInterface> {
        todo!()
    }

    pub fn getHelp(&self) -> String {
        todo!()
    }

    pub(crate) fn getDefaultCommands(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    fn getCommandNameBeforeBinding(&self, input: serde_json::Value) -> Option<String> {
        todo!()
    }

    pub fn getLongVersion(&self) -> String {
        todo!()
    }

    pub(crate) fn getDefaultInputDefinition(&self) -> serde_json::Value {
        todo!()
    }

    fn getPluginCommands(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getInitialWorkingDirectory(&self) {
        todo!()
    }

    pub fn getDisablePluginsByDefault(&self) -> bool {
        todo!()
    }

    pub fn getDisableScriptsByDefault(&self) -> bool {
        todo!()
    }

    fn getUseParentDirConfigValue(&self) {
        todo!()
    }

    fn isRunningAsRoot(&self) -> bool {
        todo!()
    }

}

