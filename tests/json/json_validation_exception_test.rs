// Namespace: Composer\Test\Json

#[derive(Debug, Clone, Default)]
pub struct JsonValidationExceptionTest {
}

impl JsonValidationExceptionTest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn testGetErrors(&self, message: String, errors: Vec<serde_json::Value>, expectedMessage: String, expectedErrors: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn testGetErrorsWhenNoErrorsProvided(&self) {
        todo!()
    }

    pub fn errorProvider() -> Vec<serde_json::Value> {
        todo!()
    }

}

