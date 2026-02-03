// Namespace: Installer

#[derive(Debug, Clone, Default)]
pub struct CommandProvider {
}

impl CommandProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getCommands(&self) {
        todo!()
    }

}

#[derive(Debug, Clone, Default)]
pub struct Command {
}

impl Command {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn configure(&self) {
        todo!()
    }

    pub(crate) fn execute(&self, input: serde_json::Value, output: serde_json::Value) -> i64 {
        todo!()
    }

}

