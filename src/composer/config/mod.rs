//! Auto-generated module declarations

pub mod config_source_interface;
pub mod json_config_source;

// --- Merged from parent module file ---

// Namespace: Composer

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct Config {
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setBaseDir(&self, baseDir: Option<String>) {
        todo!()
    }

    pub fn setConfigSource(&self, source: Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface>) {
        todo!()
    }

    pub fn getConfigSource(&self) -> Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface> {
        todo!()
    }

    pub fn setAuthConfigSource(&self, source: Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface>) {
        todo!()
    }

    pub fn getAuthConfigSource(&self) -> Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface> {
        todo!()
    }

    pub fn setLocalAuthConfigSource(&self, source: Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface>) {
        todo!()
    }

    pub fn getLocalAuthConfigSource(&self) -> Option<Box<dyn crate::composer::config::config_source_interface::ConfigSourceInterface>> {
        todo!()
    }

    pub fn merge(&self, config: Vec<serde_json::Value>, source: String) {
        todo!()
    }

    pub fn getRepositories(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn get(&self, key: String, flags: i64) {
        todo!()
    }

    pub fn all(&self, flags: i64) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn getSourceOfValue(&self, key: String) -> String {
        todo!()
    }

    fn setSourceOfConfigValue(&self, configValue: serde_json::Value, path: String, source: String) {
        todo!()
    }

    pub fn raw(&self) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn has(&self, key: String) -> bool {
        todo!()
    }

    fn process(&self, value: serde_json::Value, flags: i64) {
        todo!()
    }

    fn realpath(&self, path: String) -> String {
        todo!()
    }

    fn getComposerEnv(&self, var: String) {
        todo!()
    }

    fn disableRepoByName(&self, name: String) {
        todo!()
    }

    pub fn prohibitUrlByConfig(&self, url: String, io: Option<Box<dyn crate::composer::i_o::i_o_interface::IOInterface>>, repoOptions: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn disableProcessTimeout() {
        todo!()
    }

}

