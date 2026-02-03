// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct IOMock {
}

impl IOMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expects(&self, expectations: Vec<serde_json::Value>, strict: bool) {
        todo!()
    }

    pub fn assertComplete(&self) {
        todo!()
    }

    pub fn ask(&self, question: serde_json::Value, r#default: serde_json::Value) {
        todo!()
    }

    pub fn askConfirmation(&self, question: serde_json::Value, r#default: serde_json::Value) {
        todo!()
    }

    pub fn askAndValidate(&self, question: serde_json::Value, validator: serde_json::Value, attempts: serde_json::Value, r#default: serde_json::Value) {
        todo!()
    }

    pub fn askAndHideAnswer(&self, question: serde_json::Value) {
        todo!()
    }

    pub fn select(&self, question: serde_json::Value, choices: serde_json::Value, r#default: serde_json::Value, attempts: serde_json::Value, errorMessage: serde_json::Value, multiselect: serde_json::Value) {
        todo!()
    }

    pub fn setAuthentication(&self, repositoryName: serde_json::Value, username: serde_json::Value, password: serde_json::Value) {
        todo!()
    }

}

