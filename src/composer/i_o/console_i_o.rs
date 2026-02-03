// Namespace: Composer\IO

#[derive(Debug, Clone, Default)]
pub struct ConsoleIO {
}

impl ConsoleIO {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enableDebugging(&self, startTime: f64) {
        todo!()
    }

    pub fn isInteractive(&self) {
        todo!()
    }

    pub fn isDecorated(&self) {
        todo!()
    }

    pub fn isVerbose(&self) {
        todo!()
    }

    pub fn isVeryVerbose(&self) {
        todo!()
    }

    pub fn isDebug(&self) {
        todo!()
    }

    pub fn write(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    pub fn writeError(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    pub fn writeRaw(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    pub fn writeErrorRaw(&self, messages: serde_json::Value, newline: bool, verbosity: i64) {
        todo!()
    }

    fn doWrite(&self, messages: serde_json::Value, newline: bool, stderr: bool, verbosity: i64, raw: bool) {
        todo!()
    }

    pub fn overwrite(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64) {
        todo!()
    }

    pub fn overwriteError(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64) {
        todo!()
    }

    fn doOverwrite(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, stderr: bool, verbosity: i64) {
        todo!()
    }

    pub fn getProgressBar(&self, max: i64) {
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

    pub fn getTable(&self) -> serde_json::Value {
        todo!()
    }

    fn getErrorOutput(&self) -> serde_json::Value {
        todo!()
    }

    pub fn sanitize(messages: serde_json::Value, allowNewlines: bool) {
        todo!()
    }

    fn ensureValidUtf8(string: String) -> String {
        todo!()
    }

}

