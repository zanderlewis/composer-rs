// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct PreCommandRunEvent {
}

impl PreCommandRunEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInput(&self) -> serde_json::Value {
        todo!()
    }

    pub fn getCommand(&self) -> String {
        todo!()
    }

}

