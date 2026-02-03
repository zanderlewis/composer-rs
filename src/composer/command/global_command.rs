// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct GlobalCommand {
}

impl GlobalCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn complete(&self, input: serde_json::Value, suggestions: serde_json::Value) {
        todo!()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub fn run(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn prepareSubcommandInput(&self, input: serde_json::Value, quiet: bool) -> serde_json::Value {
        todo!()
    }

    pub fn isProxyCommand(&self) -> bool {
        todo!()
    }

}

