// Namespace: Composer\Test

#[derive(Debug, Clone, Default)]
pub struct CompletionFunctionalTest {
}

impl CompletionFunctionalTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getCommandSuggestions() -> Vec<serde_json::Value> {
        todo!()
    }

    pub fn testComplete(&self, input: String, expectedSuggestions: Option<Vec<serde_json::Value>>) {
        todo!()
    }

    fn getApplication(&self) -> crate::composer::console::application::Application {
        todo!()
    }

}

