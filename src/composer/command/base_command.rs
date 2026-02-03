// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct BaseCommand {
}

impl BaseCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getApplication(&self) -> crate::composer::console::application::Application {
        todo!()
    }

    pub fn getComposer(&self, required: bool, disablePlugins: Option<bool>, disableScripts: Option<bool>) {
        todo!()
    }

    pub fn requireComposer(&self, disablePlugins: Option<bool>, disableScripts: Option<bool>) -> crate::composer::composer::Composer {
        todo!()
    }

    pub fn tryComposer(&self, disablePlugins: Option<bool>, disableScripts: Option<bool>) -> Option<crate::composer::composer::Composer> {
        todo!()
    }

    pub fn setComposer(&self, composer: crate::composer::composer::Composer) {
        todo!()
    }

    pub fn resetComposer(&self) {
        todo!()
    }

    pub fn isProxyCommand(&self) {
        todo!()
    }

    pub fn getIO(&self) {
        todo!()
    }

    pub fn setIO(&self, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>) {
        todo!()
    }

    pub fn complete(&self, input: serde_json::Value, suggestions: serde_json::Value) {
        todo!()
    }

    pub(crate) fn initialize(&self, input: serde_json::Value, output: serde_json::Value) {
        todo!()
    }

    pub(crate) fn createComposerInstance(&self, input: serde_json::Value, io: Box<dyn crate::composer::i_o::i_o_interface::IOInterface>, config: serde_json::Value, disablePlugins: Option<bool>, disableScripts: Option<bool>) -> crate::composer::composer::Composer {
        todo!()
    }

    pub(crate) fn getPreferredInstallOptions(&self, config: crate::composer::config::Config, input: serde_json::Value, keepVcsRequiresPreferSource: bool) {
        todo!()
    }

    pub(crate) fn getPlatformRequirementFilter(&self, input: serde_json::Value) -> Box<dyn crate::composer::filter::platform_requirement_filter::platform_requirement_filter_interface::PlatformRequirementFilterInterface> {
        todo!()
    }

    pub(crate) fn formatRequirements(&self, requirements: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn normalizeRequirements(&self, requirements: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn renderTable(&self, table: Vec<serde_json::Value>, output: serde_json::Value) {
        todo!()
    }

    pub(crate) fn getTerminalWidth(&self) {
        todo!()
    }

    pub(crate) fn getAuditFormat(&self, input: serde_json::Value, optName: String) -> String {
        todo!()
    }

    pub(crate) fn createAuditConfig(&self, config: crate::composer::config::Config, input: serde_json::Value) -> crate::composer::advisory::audit_config::AuditConfig {
        todo!()
    }

}

