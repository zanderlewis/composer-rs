// Namespace: Composer\Console\Input

#[derive(Debug, Clone, Default)]
pub struct InputOption {
}

impl InputOption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn complete(&self, input: serde_json::Value, suggestions: serde_json::Value) {
        todo!()
    }

}

