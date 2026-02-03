// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct OutdatedCommand {
}

impl OutdatedCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    pub fn isProxyCommand(&self) -> bool {
        todo!()
    }

}

