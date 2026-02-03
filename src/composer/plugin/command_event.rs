// Namespace: Composer\Plugin

#[derive(Debug, Clone, Default)]
pub struct CommandEvent {
}

impl CommandEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getInput(&self) -> serde_json::Value {
        todo!()
    }

    pub fn getOutput(&self) -> serde_json::Value {
        todo!()
    }

    pub fn getCommandName(&self) -> String {
        todo!()
    }

}

