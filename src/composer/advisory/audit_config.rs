// Namespace: Composer\Advisory

#[derive(Debug, Clone, Default)]
pub struct AuditConfig {
}

impl AuditConfig {
    pub fn new() -> Self {
        Self::default()
    }

    fn parseIgnoreWithApply(config: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn fromConfig(config: crate::composer::config::Config, audit: bool, auditFormat: String) -> Self {
        todo!()
    }

}

