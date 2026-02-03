// Namespace: Composer\Command

#[derive(Debug, Clone, Default)]
pub struct AuditCommand {
}

impl AuditCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

    fn getPackages(&self, composer: crate::composer::composer::Composer, input: serde_json::Value) -> Vec<serde_json::Value> {
        todo!()
    }

}

