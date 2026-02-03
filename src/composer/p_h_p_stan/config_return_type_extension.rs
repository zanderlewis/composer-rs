// Namespace: Composer\PHPStan

#[derive(Debug, Clone, Default)]
pub struct ConfigReturnTypeExtension {
}

impl ConfigReturnTypeExtension {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getClass(&self) -> String {
        todo!()
    }

    pub fn isMethodSupported(&self, methodReflection: serde_json::Value) -> bool {
        todo!()
    }

    pub fn getTypeFromMethodCall(&self, methodReflection: serde_json::Value, methodCall: serde_json::Value, scope: serde_json::Value) -> Option<serde_json::Value> {
        todo!()
    }

    fn parseType(&self, def: Vec<serde_json::Value>, path: String) -> serde_json::Value {
        todo!()
    }

}

