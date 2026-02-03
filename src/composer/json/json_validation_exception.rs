// Namespace: Composer\Json

#[derive(Debug, Clone, Default)]
pub struct JsonValidationException {
}

impl JsonValidationException {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getErrors(&self) -> Vec<serde_json::Value> {
        todo!()
    }

}

