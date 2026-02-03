// Namespace: Composer\IO

pub trait IOInterface {
    fn isInteractive(&self);
    fn isVerbose(&self);
    fn isVeryVerbose(&self);
    fn isDebug(&self);
    fn isDecorated(&self);
    fn write(&self, messages: serde_json::Value, newline: bool, verbosity: i64);
    fn writeError(&self, messages: serde_json::Value, newline: bool, verbosity: i64);
    fn writeRaw(&self, messages: serde_json::Value, newline: bool, verbosity: i64);
    fn writeErrorRaw(&self, messages: serde_json::Value, newline: bool, verbosity: i64);
    fn overwrite(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64);
    fn overwriteError(&self, messages: serde_json::Value, newline: bool, size: Option<i64>, verbosity: i64);
    fn ask(&self, question: String, r#default: serde_json::Value);
    fn askConfirmation(&self, question: String, r#default: bool);
    fn askAndValidate(&self, question: String, validator: Box<dyn Fn()>, attempts: Option<i64>, r#default: serde_json::Value);
    fn askAndHideAnswer(&self, question: String);
    fn select(&self, question: String, choices: Vec<serde_json::Value>, r#default: serde_json::Value, attempts: serde_json::Value, errorMessage: String, multiselect: bool);
    fn getAuthentications(&self);
    fn hasAuthentication(&self, repositoryName: String);
    fn getAuthentication(&self, repositoryName: String);
    fn setAuthentication(&self, repositoryName: String, username: String, password: Option<String>);
    fn loadConfiguration(&self, config: crate::composer::config::Config);
}

