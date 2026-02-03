// Namespace: Composer\Command

// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();
// const UNKNOWN: ... = ();

#[derive(Debug, Clone, Default)]
pub struct BaseDependencyCommand {
}

impl BaseDependencyCommand {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn doExecute(&self, input: serde_json::Value, output: serde_json::Value, inverted: bool) -> i64 {
        todo!()
    }

    pub(crate) fn printTable(&self, output: serde_json::Value, results: Vec<serde_json::Value>) {
        todo!()
    }

    pub(crate) fn initStyles(&self, output: serde_json::Value) {
        todo!()
    }

    pub(crate) fn printTree(&self, results: Vec<serde_json::Value>, prefix: String, level: i64) {
        todo!()
    }

    fn writeTreeLine(&self, line: String) {
        todo!()
    }

}

