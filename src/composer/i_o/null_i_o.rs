// Namespace: Composer\IO

#[derive(Debug, Clone, Default)]
pub struct NullIO {
}

impl NullIO {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn isInteractive(&self) -> bool {
        todo!()
    }

    pub fn isVerbose(&self) -> bool {
        todo!()
    }

    pub fn isVeryVerbose(&self) -> bool {
        todo!()
    }

    pub fn isDebug(&self) -> bool {
        todo!()
    }

    pub fn isDecorated(&self) -> bool {
        todo!()
    }

    pub fn write(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    pub fn writeError(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    pub fn overwrite(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64) {
        todo!()
    }

    pub fn overwriteError(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64) {
        todo!()
    }

    pub fn ask(&self, question: serde_json::Value, r#default: serde_json::Value) {
        todo!()
    }

    pub fn askConfirmation(&self, question: serde_json::Value, r#default: serde_json::Value) -> bool {
        todo!()
    }

    pub fn askAndValidate(&self, question: serde_json::Value, validator: serde_json::Value, attempts: serde_json::Value, r#default: serde_json::Value) {
        todo!()
    }

    pub fn askAndHideAnswer(&self, question: serde_json::Value) -> Option<String> {
        todo!()
    }

    pub fn select(&self, question: serde_json::Value, choices: serde_json::Value, r#default: serde_json::Value, attempts: serde_json::Value, errorMessage: serde_json::Value, multiselect: serde_json::Value) {
        todo!()
    }

}

