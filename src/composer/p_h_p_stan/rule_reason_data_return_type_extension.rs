// Namespace: Composer\PHPStan

#[derive(Debug, Clone, Default)]
pub struct RuleReasonDataReturnTypeExtension {
}

impl RuleReasonDataReturnTypeExtension {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getClass(&self) -> String {
        todo!()
    }

    pub fn isMethodSupported(&self, methodReflection: serde_json::Value) -> bool {
        todo!()
    }

    pub fn getTypeFromMethodCall(&self, methodReflection: serde_json::Value, methodCall: serde_json::Value, scope: serde_json::Value) -> serde_json::Value {
        todo!()
    }

}

