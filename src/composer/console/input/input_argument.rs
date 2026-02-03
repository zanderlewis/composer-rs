// Namespace: Composer\Console\Input

#[derive(Debug, Clone, Default)]
pub struct InputArgument {
}

impl InputArgument {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn complete(&self, input: serde_json::Value, suggestions: serde_json::Value) {
        todo!()
    }

}

