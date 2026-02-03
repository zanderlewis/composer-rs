// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct RunScriptCommand {
}

impl RunScriptCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn interact(&self, input: serde_json::Value, output: serde_json::Value) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    pub(crate) fn listScripts(&self, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn getScripts(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

