// Namespace: Composer\Test\Command

#[derive(Debug, Clone, Default)]
pub struct SearchCommandTest {
}

impl SearchCommandTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testSearch(&self, command: Vec<serde_json::Value>, expected: String) {
        todo!()
    }

    pub fn testInvalidFormat(&self) {
        todo!()
    }

    pub fn testInvalidFlags(&self) {
        todo!()
    }

    pub fn provideSearch() -> serde_json::Value {
        todo!()
    }

}

