// Namespace: Composer\Test\Mock

#[derive(Debug, Clone, Default)]
pub struct ProcessExecutorMock {
}

impl ProcessExecutorMock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expects(&self, expectations: Vec<serde_json::Value>, strict: bool, defaultHandler: Vec<serde_json::Value>) {
        todo!()
    }

    pub fn assertComplete(&self) {
        todo!()
    }

    pub fn execute(&self, command: serde_json::Value, output: serde_json::Value, cwd: Option<String>) -> i64 {
        todo!()
    }

    pub fn executeTty(&self, command: serde_json::Value, cwd: Option<String>) -> i64 {
        todo!()
    }

    fn doExecute(&self, command: serde_json::Value, cwd: String, tty: bool, output: serde_json::Value) {
        todo!()
    }

    pub fn executeAsync(&self, command: serde_json::Value, cwd: Option<String>) -> serde_json::Value {
        todo!()
    }

    pub fn getErrorOutput(&self) -> String {
        todo!()
    }

}

